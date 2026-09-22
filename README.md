[![CICD](https://github.com/tupl-dev/sectxt/actions/workflows/cicd.yml/badge.svg)](https://github.com/tupl-dev/sectxt/actions/workflows/cicd.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

# Sectxt
[PrivateBin](https://privatebin.info/) clone written in [Rust](https://rust-lang.org/).

## Backend Crates
- [`sectxt-api`](backend/crates/sectxt-api/): API layer
- [`sectxt-core`](backend/crates/sectxt-core/): domain layer
- [`sectxt-db`](backend/crates/sectxt-db/): database layer

## Development
Required tools:
- [`cargo`](https://rust-lang.org/tools/install/)
- [`docker`](https://www.docker.com/products/docker-desktop/)
- [`npm`](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli)

```shell
docker compose up -d database
cp .env.sample .env

# backend
sqlx migrate run --source 'backend/crates/sectxt-db/migrations'
cargo test 'generate_openapi_doc' --manifest-path 'backend/Cargo.toml'
cargo run --manifest-path 'backend/Cargo.toml'

# frontend
npm --prefix 'frontend/' install && npm --prefix 'frontend/' run dev
```

## Deployment
Required tools:
- [`cargo`](https://rust-lang.org/tools/install/)
- [`cross`](https://crates.io/crates/cross)
- [`docker`](https://www.docker.com/products/docker-desktop/)
- [`npm`](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli)
- [`terraform`](https://developer.hashicorp.com/terraform/install)

### Setup Infrastructure
```shell
export DIGITALOCEAN_TOKEN='<DIGITALOCEAN_TOKEN>'
export NEON_API_KEY='<NEON_API_KEY>'
ssh-keygen -t ed25519 -f ~/.ssh/keys/id_ed25519_sectxt
terraform init
terraform apply -auto-approve
sqlx migrate run --source 'backend/crates/sectxt-db/migrations' --database-url "$(terraform output -raw database_url_direct)"
```
Update the DNS on your domain to point to droplet's IP (`terraform output -raw server_ip`).

### Setup Site
Update `VITE_API_BASE_URL` to `https://<your domain>/api` so frontend can reach the backend.
```shell
cp .env.sample .env
cross build --release --target 'x86_64-unknown-linux-gnu' --manifest-path 'backend/Cargo.toml' --target-dir 'dist/backend/'
npm --prefix 'frontend/' install && npm --prefix 'frontend/' run build
docker compose run --build --rm ansible ansible-playbook -i inventory.ini deploy.yml

# check logs
ssh deployer@$(terraform output -raw server_ip) -i ~/.ssh/keys/id_ed25519_sectxt 'sudo journalctl -u sectxt-api --no-pager -n 500'
```

### Destroy
```shell
terraform destroy -auto-approve
```
