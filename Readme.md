https://youtu.be/3cA_mk4vdWY?t=4738

https://youtu.be/3cA_mk4vdWY?t=33

cargo watch -q -c -w src/ -w .cargo/ -x run

cargo watch -q -c -w examples/ -x "run --example quickdev"

cargo watch -q -c -x "test -- --nocapture"

docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15

sqlx version > 0.6 seems to have an issue with pools, set max connections to 1