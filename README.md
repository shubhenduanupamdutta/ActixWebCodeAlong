# Code Along For [YouTub Actix Web Playlist](https://www.youtube.com/watch?v=aZmrfizffL0&list=PLGOIZXklfFkRh8jHNY8070KUl86Tj3Ztf) by **OptiCode**


---
## Sea-ORM Migration setup

### Installing Sea-ORM-CLI for managing migrations

```bash
cargo install sea-orm-cli
sea-orm-cli migrate init
```
### Generating a migration
```bash
sea-orm-cli migrate generate create_user_table
```
This will create migration with default post table.

### Running a migration
```bash
sea-orm-cli migrate up
```

## Sea-ORM Entity Generation

If `DATABASE_URL` is defined in .env file. Then you can auto generate entities using
```bash
sea-orm-cli generate entity -o entity/src
```

Otherwise

```bash
sea-orm-cli generate entity -u protocol:://user:password@endpoint_url/database -o entity/src
```
