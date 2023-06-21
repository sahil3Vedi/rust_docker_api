# JWT Authentication in Rust

# 🚧 This project is still under construction

## Dependencies
Docker & docker-compose

## Setup
Clone the repo and enter the root directory of the project
```
docker-compose down && docker-compose build && docker-compose up -d
```

## Endpoints:
- `localhost:4000/register`: Register User
- `localhost:4000/login`: Login User
- `localhost:4000/delete`: Delete User
- `localhost:4001/view`: View Users (Admin Only)
- `localhost:4002/kick`: Kick User (Admin Only)