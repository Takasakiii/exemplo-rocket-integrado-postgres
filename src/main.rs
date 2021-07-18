use rocket::{
    get, launch, post,
    response::status,
    routes,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_sync_db_pools::{database, postgres};

#[database("postgres_db")]
struct MainDatabase(postgres::Client);

#[derive(Deserialize, Clone, Serialize)]
struct Carro {
    marca: String,
    modelo: String,
    placa: String,
}

#[launch]
fn start() -> _ {
    rocket::build()
        .attach(MainDatabase::fairing())
        .mount("/carros", routes![add, list])
}

#[post("/", data = "<carro>", format = "json")]
async fn add(
    conn: MainDatabase,
    carro: Json<Carro>,
) -> Result<Json<Carro>, status::BadRequest<()>> {
    let result = carro.clone();

    conn.run(move |c| {
        c.execute(
            "insert into carro values ($1, $2, $3)",
            &[&carro.marca, &carro.modelo, &carro.placa],
        )
    })
    .await
    .map_err(|_| status::BadRequest(None))?;

    Ok(Json(result))
}

#[get("/", format = "json")]
async fn list(conn: MainDatabase) -> Json<Vec<Carro>> {
    let carros = conn
        .run(|c| {
            let mut carros = Vec::new();
            for row in c.query("select * from carro", &[]).unwrap() {
                let carro = Carro {
                    marca: row.get(0),
                    modelo: row.get(1),
                    placa: row.get(2),
                };
                carros.push(carro);
            }
            carros
        })
        .await;

    Json(carros)
}
