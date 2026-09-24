# Todo List API

A simple Todo List REST API built with **Rust**, **Axum**, **SQLx**, and **SQLite**, containerized with Docker and Docker Compose.

## Features

* Create, read, update, and delete todos
* SQLite database
* Database migrations
* Dockerized Rust application
* Docker Compose for easy deployment
* Persistent database storage using a Docker volume
* Health check endpoint
* Environment-based database configuration

## Project Structure

```text
todo-api/
├── src/
│   └── main.rs
├── migrations/
│   └── ...
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
├── docker-compose.yml
└── README.md
```

## API Endpoints

| Method | Endpoint     | Description      |
| ------ | ------------ | ---------------- |
| GET    | `/health`    | Check API health |
| GET    | `/todos`     | Get all todos    |
| POST   | `/todos`     | Create a todo    |
| GET    | `/todos/:id` | Get one todo     |
| PUT    | `/todos/:id` | Update a todo    |
| DELETE | `/todos/:id` | Delete a todo    |

### Todo Format

```json
{
  "id": 1,
  "title": "Learn Rust",
  "completed": false
}
```

---

# Running with Docker Compose

## 1. Build and Start the Application

From the project directory:

```bash
docker compose up --build
```

The application should be available at:

```text
http://localhost:3000
```

To run it in the background:

```bash
docker compose up --build -d
```

## 2. Check the Containers

```bash
docker compose ps
```

You should see the Todo API container running.

## 3. Check the Logs

```bash
docker compose logs
```

Or:

```bash
docker compose logs -f
```

---

# Testing the API

## Health Check

```bash
curl http://localhost:3000/health
```

Expected response:

```json
{
  "status": "healthy"
}
```

## Get All Todos

```bash
curl http://localhost:3000/todos
```

If there are no todos:

```json
[]
```

## Create a Todo

```bash
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"Learn Rust","completed":false}'
```

Example response:

```json
{
  "id": 1,
  "title": "Learn Rust",
  "completed": false
}
```

## Get a Todo

```bash
curl http://localhost:3000/todos/1
```

## Update a Todo

```bash
curl -X PUT http://localhost:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Learn Rust and Docker","completed":true}'
```

## Delete a Todo

```bash
curl -X DELETE http://localhost:3000/todos/1
```

---

# Database Persistence

The application uses SQLite to store todos.

The database is stored inside the container, but the database directory is mounted to a Docker volume so that data survives container restarts and recreation.

The Docker Compose configuration should use a named volume similar to:

```yaml
volumes:
  todos-data:
```

and mount it to the database directory inside the container.

For example:

```yaml
volumes:
  - todos-data:/data
```

This means:

```text
Docker Volume
    ↓
/data
    ↓
SQLite database
```

### Important

Stopping or removing the container does **not** delete the named volume.

For example:

```bash
docker compose down
```

removes the containers but keeps the named volume unless the volume is explicitly removed.

To remove the containers **and the database volume**:

```bash
docker compose down -v
```

**Warning:** `docker compose down -v` deletes the persistent database volume and therefore removes the stored Todo data.

---

# Environment Variables

The application can use an environment variable to specify the database location.

Example:

```text
DATABASE_URL=sqlite:/data/todos.db
```

The database path should match the location mounted by Docker Compose.

---

# Docker Commands

## Build the Image

```bash
docker compose build
```

## Start the Application

```bash
docker compose up
```

## Start in Background

```bash
docker compose up -d
```

## Rebuild the Application

```bash
docker compose up --build
```

## Stop the Application

```bash
docker compose stop
```

## Stop and Remove Containers

```bash
docker compose down
```

## Stop, Remove Containers and Volumes

```bash
docker compose down -v
```

Use this only when you intentionally want to delete the database.

---

# Checking Database Volume

List Docker volumes:

```bash
docker volume ls
```

Inspect the Todo volume:

```bash
docker volume inspect todos-data
```

The volume should remain available even after:

```bash
docker compose down
```

unless `-v` is used.

---

# Docker Health Check

The application exposes:

```text
GET /health
```

Docker uses this endpoint to determine whether the application is healthy.

Check container status:

```bash
docker compose ps
```

A healthy container should show:

```text
healthy
```

---

# Development

If Rust is installed locally, the application can also be run without Docker.

Install dependencies and build:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

The API should then be available at:

```text
http://localhost:3000
```

---

# Technologies

* **Rust** – Application programming language
* **Axum** – Web framework
* **SQLx** – Database access
* **SQLite** – Database
* **Docker** – Containerization
* **Docker Compose** – Container orchestration

---

# Troubleshooting

## Container Does Not Start

Check the logs:

```bash
docker compose logs
```

For the application container specifically:

```bash
docker compose logs <service-name>
```

## Port 3000 Already in Use

Check what is using port 3000:

```bash
docker ps
```

Stop the conflicting container or change the host port in `docker-compose.yml`.

For example:

```yaml
ports:
  - "3001:3000"
```

The API would then be accessed through:

```text
http://localhost:3001
```

## Todos Disappear

Make sure the database directory is mounted to a named Docker volume.

Check:

```bash
docker volume ls
```

Do not run:

```bash
docker compose down -v
```

if you want to keep the existing Todo data.

---

