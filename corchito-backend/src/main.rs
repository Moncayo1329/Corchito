use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod routes; // Import the routes module
use crate::routes::usuario::{Usuario, listar_usuarios, login, crear_usuario, ver_usuario};

#[actix_web::main]
// esto marca la funcion main como punto de entrada. 
async fn main() -> std::io::Result<()>{
    println!("Servidor corchito iniciando...");



/// Aqui arrancamos un servidor http en la direccion 127.0.0.1:8080


HttpServer::new(|| {

App::new()
// definimos una ruta POST en "/login" 

.route("/login", web::post().to(login_endpoint))
// definimos una ruta GET en "/usuarios" para listar 
.route("/usuarios", web::get().to(listar_usuarios_endpoint))
}) 

.bind("127.0.0.1:8080")? // Vinculamos el servidor a un puerto local. 
.run()
.await
}
// =====================
// ENDPOINTS HTTP
// =====================

// Recibe JSON( email y password) y responde si el login es valido 
async fn login_endpoint(info: web::Json<Usuario>) -> impl Responder {

    let usuarios = listar_usuarios(); // Obtener lista de prueba. 


// Buscamos si el usuario existe. 

let intento = usuarios.into_iter()
.find(|u| u.email == info.email && u.password == info.password);

match intento {
    Some(u) => HttpResponse::Ok().json(u), // 200 ok mas datos en json
    None => HttpResponse::Unauthorized().body("Email o password incorrectos"),
}

}


// Endpoint para listar todos los usuarios
async fn listar_usuarios_endpoint() -> impl Responder {
    let usuarios = listar_usuarios();
    HttpResponse::Ok().json(usuarios) // devolvemos un JSON con el vector completo
}

