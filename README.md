# Gym Helper API 🏋️‍♂️

Una API REST completa para la gestión de gimnasios desarrollada en Rust con Actix-web. Sistema integral para administrar clientes, membresías, disciplinas y suscripciones.

## 🚀 Características

- **Gestión de Clientes**: CRUD completo para administrar clientes del gimnasio
- **Sistema de Membresías**: Administración de disciplinas y planes de membresía
- **Suscripciones**: Control de suscripciones activas y asistencia a clases
- **Autenticación JWT**: Sistema de autenticación seguro con roles
- **Documentación OpenAPI**: Swagger UI integrado para explorar la API
- **Tests Unitarios**: Suite completa de tests (41 tests implementados)

## 📋 Requisitos Previos

- **Rust** (versión 1.70 o superior)
- **MySQL** (versión 8.0 o superior)
- **Cargo** (incluido con Rust)

## 🛠️ Instalación

1. **Clonar el repositorio**
   ```bash
   git clone <url-del-repositorio>
   cd gym_helper
   ```

2. **Configurar la base de datos**
   ```bash
   # Crear la base de datos
   mysql -u root -p
   CREATE DATABASE gym_helper;
   ```

3. **Configurar variables de entorno**
   
   Editar el archivo `env/env.json`:
   ```json
   {
       "api_bind": "127.0.0.1:8080",
       "database_url": "mysql://usuario:contraseña@localhost:3306/gym_helper",
       "jwt_secret": "tu_clave_secreta_jwt"
   }
   ```

4. **Ejecutar migraciones**
   ```bash
   # Las migraciones se encuentran en la carpeta migrations/
   # Ejecutar manualmente los archivos SQL en orden:
   # - 20250419233445_initial_migration.sql
   # - 20250507020431_create_membership_schema.sql
   ```

5. **Instalar dependencias y compilar**
   ```bash
   cargo build
   ```

## 🚀 Ejecución

```bash
# Ejecutar en modo desarrollo
cargo run

# Ejecutar con logs detallados
RUST_LOG=debug cargo run

# Ejecutar tests
cargo test
```

La API estará disponible en: `http://localhost:8080`

## 📚 Documentación API

### Swagger UI
Accede a la documentación interactiva en: `http://localhost:8080/swagger-ui/`

### OpenAPI JSON
Especificación OpenAPI disponible en: `http://localhost:8080/api-docs/openapi.json`

## 🏗️ Estructura del Proyecto

```
gym_helper/
├── src/
│   ├── auth/                 # Módulo de autenticación
│   │   ├── handlers.rs       # Handlers de autenticación
│   │   ├── jwt.rs           # Utilidades JWT
│   │   ├── middleware.rs    # Middleware de autenticación
│   │   ├── services.rs      # Servicios de autenticación
│   │   └── models/          # Modelos de autenticación
│   ├── clients/             # Módulo de clientes
│   │   ├── handlers.rs      # Lógica de base de datos
│   │   ├── services.rs      # Endpoints REST
│   │   └── models/          # Modelos y requests
│   ├── membership/          # Módulo de membresías
│   │   ├── handlers.rs      # Lógica de base de datos
│   │   ├── services.rs      # Endpoints REST
│   │   └── models/          # Modelos y requests
│   ├── subscription/        # Módulo de suscripciones
│   │   ├── handlers.rs      # Lógica de base de datos
│   │   ├── services.rs      # Endpoints REST
│   │   └── models.rs        # Modelos y requests
│   ├── config.rs            # Configuración de la aplicación
│   ├── db.rs               # Configuración de base de datos
│   ├── openapi.rs          # Configuración OpenAPI/Swagger
│   └── main.rs             # Punto de entrada
├── migrations/              # Migraciones de base de datos
├── env/                    # Archivos de configuración
└── Cargo.toml             # Dependencias del proyecto
```

## 🔗 Endpoints Principales

### Autenticación
- `POST /auth/login` - Iniciar sesión
- `POST /auth/register` - Registrar usuario

### Clientes
- `GET /clients` - Listar todos los clientes
- `GET /clients/{id}` - Obtener cliente por ID
- `GET /clients/filter` - Filtrar clientes con parámetros
- `POST /clients` - Crear nuevo cliente
- `PUT /clients/{id}` - Actualizar cliente (admin)
- `PATCH /clients/{id}` - Activar cliente
- `DELETE /clients/{id}` - Eliminar cliente

### Membresías y Disciplinas
- `POST /membership/discipline` - Crear disciplina
- `DELETE /membership/discipline/{id}` - Eliminar disciplina
- `PATCH /membership/discipline/{id}` - Activar disciplina
- `POST /membership` - Crear membresía
- `DELETE /membership/{id}` - Eliminar membresía
- `PATCH /membership/{id}` - Activar membresía

### Suscripciones
- `POST /subscription` - Crear suscripción
- `GET /subscription/client/{id}` - Obtener suscripciones de cliente
- `POST /subscription/attendance` - Registrar asistencia
- `DELETE /subscription/{id}` - Eliminar suscripción

## 🧪 Testing

El proyecto incluye una suite completa de tests unitarios:

```bash
# Ejecutar todos los tests
cargo test

# Ejecutar tests con output detallado
cargo test -- --nocapture

# Ejecutar tests de un módulo específico
cargo test subscription::tests
cargo test clients::tests
cargo test membership::tests
```

### Cobertura de Tests
- **Subscription Services**: 11 tests
- **Clients Services**: 13 tests  
- **Membership Services**: 17 tests
- **Total**: 41 tests unitarios

## 🔧 Tecnologías Utilizadas

- **[Actix-web](https://actix.rs/)** - Framework web async
- **[SQLx](https://github.com/launchbadge/sqlx)** - Driver de base de datos async
- **[Serde](https://serde.rs/)** - Serialización/deserialización
- **[Chrono](https://github.com/chronotope/chrono)** - Manejo de fechas
- **[Tracing](https://tracing.rs/)** - Logging estructurado
- **[JsonWebToken](https://github.com/Keats/jsonwebtoken)** - Autenticación JWT
- **[Utoipa](https://github.com/juhaku/utoipa)** - Documentación OpenAPI
- **[Argon2](https://github.com/RustCrypto/password-hashes)** - Hashing de contraseñas

## 🔐 Seguridad

- Autenticación basada en JWT
- Hashing seguro de contraseñas con Argon2
- Middleware de autorización por roles
- Validación de entrada en todos los endpoints

## 📝 Modelos de Datos

### Cliente
```rust
struct Client {
    id: i32,
    name: String,
    last_name: String,
    age: i32,
    phone: String,
    active: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}
```

### Disciplina
```rust
struct Discipline {
    id: i32,
    name: String,
    description: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}
```

### Membresía
```rust
struct Membership {
    id: i32,
    name: String,
    description: Option<String>,
    price: f64,
    discipline_id: i32,
    total_classes: i32,
    active: bool,
    duration_days: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}
```

### Suscripción
```rust
struct Subscription {
    id: i32,
    client_id: i32,
    membership_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    total_classes: i32,
    remaining_classes: i32,
    class_attendance: i32,
    active: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}
```

## 🚨 Solución de Problemas Comunes

### Error: "Address already in use"
```bash
# En Windows PowerShell, encontrar y terminar el proceso:
netstat -ano | findstr :8080
taskkill /PID <PID> /F

# O cambiar el puerto en env/env.json
```

### Error de conexión a MySQL
```bash
# Verificar que MySQL esté corriendo:
net start mysql80  # Windows
sudo systemctl start mysql  # Linux

# Verificar credenciales en env/env.json
```

### Tests fallando
```bash
# Ejecutar tests con más información:
cargo test -- --nocapture

# Limpiar y recompilar:
cargo clean && cargo build
```

## 🤝 Contribución

1. Fork el proyecto
2. Crear una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abrir un Pull Request

## 👨‍💻 Autor

**Gym Helper Team**
- Email: aguswaizman98@gmail.com

## 🙏 Agradecimientos

- Comunidad de Rust por las excelentes herramientas
- Actix-web por el framework robusto
- Todos los contribuidores de las dependencias utilizadas

---

⭐ ¡Dale una estrella al proyecto si te ha sido útil! 