# Usage

```
drinks-service --postgres-host <POSTGRES_HOST> --postgres-user <POSTGRES_USER> --postgres-password <POSTGRES_PASSWORD> --postgres-database <POSTGRES_DATABASE> --listen <LISTEN>

Options:
      --postgres-host <POSTGRES_HOST>          [env: POSTGRES_HOST=]
  -u, --postgres-user <POSTGRES_USER>          [env: POSTGRES_USER=]
  -p, --postgres-password <POSTGRES_PASSWORD>  [env: POSTGRES_PASSWORD=]
  -d, --postgres-database <POSTGRES_DATABASE>  [env: POSTGRES_DATABASE=]
  -l, --listen <LISTEN>                        [env: LISTEN=]
  -h, --help                                   Print help
```
# Building the Docker Image

```sh
docker build -t drinks-service .
```
