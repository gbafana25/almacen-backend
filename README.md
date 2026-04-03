# almacen-backend

- SeaORM tutorial: https://www.sea-ql.org/sea-orm-tutorial/ch01-02-migration-cli.html
- SeaORM+Rocket example: https://github.com/SeaQL/sea-orm-tutorial/tree/master/rocket-example
- Rocket guide: https://rocket.rs/guide/v0.5/

## Running
- run `cargo build` in `almacen-backend/`
- run docker with `docker compose up --build`
- run `DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db sea-orm-cli migrate up -d ./src/migration/`

## TODO
- move request objects/endpoints into separate files (`api/` folder)
- figure out more automated way of updating migrations in docker compose
- find another way to serialize object data when returned in requests (Serialize/Deserialize flags in entity files get overwritten)