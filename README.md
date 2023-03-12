# Buffett

Stock market notifictaion system.

## Installation

1) Create new file `./conf.toml` from template `conf_template.toml`.
2) Define path to SQLite file in created cofiguration file.
3) Build the SQLite file and create database schema.

### Database schema
Create sqlite lite db and 
```sql
CREATE TABLE stock_trash_hold (
    id INTEGER PRIMARY KEY,
    ticker VARCHAR(4),
    greather_than FLOAT,
    less_than FLOAT
);
```
