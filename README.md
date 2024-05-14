# Sovereign Chess Backend Service

Backend built in Rust, using Axum for the web server.

To run locally:

```bash
source .env
cargo run
```

To build binary:

```bash
cargo build --release --verbose
```

To deploy:

```bash
# Build Docker image.  You do not need to run the above build command.
./bin/predeploy.sh

# Use Ansible to deploy
cd ../devops && ./bin/sochess_deploy.sh
```
