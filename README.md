# almacen-backend

- SeaORM tutorial: https://www.sea-ql.org/sea-orm-tutorial/ch01-02-migration-cli.html
- SeaORM+Rocket example: https://github.com/SeaQL/sea-orm-tutorial/tree/master/rocket-example
- Rocket guide: https://rocket.rs/guide/v0.5/

## Running
- run `cargo build` in `almacen-backend/`
- run docker with `docker compose up --build`
- run `DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db sea-orm-cli migrate up -d ./src/migration/`
- run `sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/app_db -o src/entities/` to generate entities

## TODO
- setup cors
    - https://docs.rs/rocket_cors/latest/rocket_cors/
    - https://github.com/steadylearner/code/blob/master/post/Rust/How%20to%20use%20CORS%20with%20Rust%20Rocket.md
- figure out more automated way of updating migrations in docker compose
