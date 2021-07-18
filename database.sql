create table carro (
	marca varchar(30) not null,
	modelo varchar(30) not null,
	placa varchar(7) not null,
	primary key(marca, modelo, placa)
);