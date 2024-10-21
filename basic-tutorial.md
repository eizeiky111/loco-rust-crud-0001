
first time user run this command: 
1. cargo install loco-cli
2. cargo install sea-orm-cli
3. loco new

4.
✔ ❯ App name? · myapp
✔ ❯ What would you like to build? · SaaS app (with DB and user auth)
✔ ❯ Select a DB Provider · Sqlite
✔ ❯ Select your background worker type · Async (in-process tokio async tasks)
✔ ❯ Select an asset serving configuration · Client (configures assets for frontend serving)

🚂 Loco app generated successfully in:
myapp/


5. cargo fmt --all -- --check ( checking style )
6. cargo fmt ( to run auto fix style )
7. cargo clippy --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery -W rust-2018-idioms
7. cargo clippy --fix --lib -p loco_crud ( implement suggestion )
8. cargo test --all-features --all ( running testcase   )

=================================================================================
cargo watch -x run -- cargo loco start --> auto compiled

1. Create new Controller API
cargo loco generate controller guide -k api

2. Create new Entity models table
cargo loco generate model article title:string content:text

3. Create new Controller Articles
cargo loco generate controller articles -k api

4. Create relationship foreign key
cargo loco generate scaffold comment content:text article:references --api

5. To see current task
cargo loco task

6. Generate Seeding task
cargo loco generate task user_reports

7. Run Task
cargo loco task user_report var1:val1 var2:val2
cargo run task seed_data refresh:true
cargo run task seed_articles