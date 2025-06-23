use utoipa::OpenApi;

// Importar todos los modelos necesarios
use crate::subscription::models::{
    Subscription, NewSubscriptionRequest, ClassAttendanceRequest, SubscriptionQueryParams
};
use crate::clients::models::{
    clients::Client,
    requests::{CreateClientRequest, ClientQueryParams}
};
use crate::membership::models::{
    membership::{Discipline, Membership},
    requests::{NewDisciplineRequest, NewMembershipRequest}
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Gym Helper API",
        version = "1.0.0",
        description = "API para la gestión de gimnasios - Sistema completo para administrar clientes, membresías y suscripciones",
        contact(
            name = "Gym Helper Team",
            email = "contacto@gymhelper.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    components(
        schemas(
            // Subscription schemas
            Subscription,
            NewSubscriptionRequest,
            ClassAttendanceRequest,
            SubscriptionQueryParams,
            
            // Client schemas
            Client,
            CreateClientRequest,
            ClientQueryParams,
            
            // Membership schemas
            Discipline,
            Membership,
            NewDisciplineRequest,
            NewMembershipRequest,
        )
    ),
    tags(
        (name = "Subscriptions", description = "Operaciones relacionadas con suscripciones de clientes"),
        (name = "Clients", description = "Gestión de clientes del gimnasio"),
        (name = "Memberships", description = "Administración de membresías y disciplinas"),
    ),
    servers(
        (url = "http://localhost:8080", description = "Servidor de desarrollo"),
        (url = "https://api.gymhelper.com", description = "Servidor de producción")
    )
)]
pub struct ApiDoc; 