# Project Hierarchy

Generated on: 2025-01-13 09:25:07

### Structure

```
./
|-- configuration/
|   |-- base.yaml
|   |-- local.yaml
|   `-- production.yaml
|-- docs/
|   |-- ai-slop/
|   |   |-- genesis.md
|   |   |-- old-README.md
|   |   `-- protocols.md
|   |-- episode-transcriptions/
|   |   |-- 001.md
|   |   |-- 095.md
|   |   |-- 138.md
|   |   |-- 139.md
|   |   `-- 140.md
|   |-- configuration.md
|   |-- hierarchy.md
|   |-- htmx-nostr-chat.md
|   |-- newsletter.md
|   `-- rust-setup.md
|-- migrations/
|   |-- 20250110000000_initial.sql
|   |-- 20250112001624_create_subscriptions_table.sql
|   `-- 20250112002000_create_agent_tables.sql
|-- scripts/
|   `-- generate_hierarchy.sh*
|-- src/
|   |-- agents/
|   |   |-- agent.rs
|   |   |-- manager.rs
|   |   `-- mod.rs
|   |-- nostr/
|   |   |-- db.rs
|   |   |-- event.rs
|   |   |-- mod.rs
|   |   |-- relay.rs
|   |   `-- subscription.rs
|   |-- routes/
|   |   |-- home/
|   |   |   |-- home.html
|   |   |   `-- mod.rs
|   |   `-- mod.rs
|   |-- server/
|   |   |-- admin/
|   |   |   |-- middleware.rs
|   |   |   |-- mod.rs
|   |   |   `-- routes.rs
|   |   |-- config.rs
|   |   |-- mod.rs
|   |   `-- routes.rs
|   |-- configuration.rs
|   |-- database.rs
|   |-- emailoptin.rs
|   |-- lib.rs
|   `-- main.rs
|-- static/
|   |-- css/
|   |   |-- changelog.css
|   |   |-- chat.css
|   |   |-- style.css
|   |   |-- videos-new.css
|   |   `-- videos.css
|   |-- data/
|   |   |-- changelog.json
|   |   `-- videos.json
|   |-- dist/
|   |   |-- nostr/
|   |   |   |-- nostr-agent.js
|   |   |   |-- nostr-agent.js.map
|   |   |   |-- nostr-chat.js
|   |   |   `-- nostr-chat.js.map
|   |   |-- ndk.js
|   |   |-- ndk.js.map
|   |   |-- nostr-sub.js
|   |   `-- nostr-sub.js.map
|   |-- docs/
|   |   `-- hierarchy.md
|   |-- fonts/
|   |   |-- BerkeleyMono-Bold.woff
|   |   |-- BerkeleyMono-Bold.woff2
|   |   |-- BerkeleyMono-BoldItalic.woff
|   |   |-- BerkeleyMono-BoldItalic.woff2
|   |   |-- BerkeleyMono-Italic.woff
|   |   |-- BerkeleyMono-Italic.woff2
|   |   |-- BerkeleyMono-Regular.woff
|   |   `-- BerkeleyMono-Regular.woff2
|   |-- js/
|   |   |-- LightingSystem.js
|   |   |-- OnyxOrb.js
|   |   |-- SceneSystem.js
|   |   |-- ViewSystem.js
|   |   |-- client-side-templates.js
|   |   |-- htmx.min.js
|   |   |-- layout.js
|   |   |-- main.js
|   |   |-- mustache.js
|   |   `-- three.min.js
|   |-- layout/
|   |   |-- footer.html
|   |   `-- header.html
|   |-- nostr/
|   |   |-- agent-methods.ts
|   |   |-- base.ts
|   |   |-- channel-methods.ts
|   |   |-- example.html
|   |   |-- message-methods.ts
|   |   |-- nostr-agent.ts
|   |   |-- nostr-chat.js
|   |   |-- nostr-chat.ts
|   |   |-- storage.ts
|   |   `-- types.ts
|   |-- templates/
|   |   |-- button.mustache
|   |   |-- changelog-new.mustache
|   |   |-- changelog.mustache
|   |   |-- chat.mustache
|   |   |-- header.json
|   |   `-- textinput.mustache
|   |-- README.md
|   |-- agents.html
|   |-- business.html
|   |-- changelog.html
|   |-- company.html
|   |-- contact.html
|   |-- favicon.ico
|   |-- index-old.html
|   |-- index.html
|   |-- justfile
|   |-- mobile-app.html
|   |-- ndk.ts
|   |-- new-backup.html
|   |-- nostr-sub.ts
|   |-- nostr.html
|   |-- onyx.png
|   |-- package.json
|   |-- styles.css
|   |-- tsconfig.json
|   `-- yarn.lock
|-- templates/
|   |-- admin/
|   |   |-- dashboard.html
|   |   `-- login.html
|   |-- layouts/
|   |   `-- base.html
|   `-- pages/
|       |-- home.html
|       `-- video-series.html
|-- tests/
|   |-- agent/
|   |   |-- core.rs
|   |   |-- manager.rs
|   |   |-- manager_comprehensive.rs
|   |   |-- manager_impl.rs
|   |   |-- mod.rs
|   |   `-- nostr.rs
|   |-- nostr/
|   |   |-- database.rs
|   |   |-- event.rs
|   |   |-- mod.rs
|   |   `-- subscription.rs
|   |-- admin_middleware.rs
|   |-- admin_routes.rs
|   |-- agent.rs
|   |-- emailoptin.rs
|   `-- health_check.rs
|-- Cargo.lock
|-- Cargo.toml
|-- DEVELOPMENT.md
|-- Dockerfile
|-- README.md
|-- html
`-- spec.yaml

32 directories, 141 files
```
