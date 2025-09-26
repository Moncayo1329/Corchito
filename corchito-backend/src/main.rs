use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod routes; // Import the routes module
use crate::routes::usuario::{listar_usuarios, login, crear_usuario, ver_usuario};

#[actix_web::main]
// esto marca la funcion main como punto de entrada. 
asyn fn main() -> std::io::Result<()>{
    println!("Servidor corchito iniciando...");

    // Create a mutable vector for usuarios
    let mut usuarios = listar_usuarios(); // Use the module-level function

    // Probar login
    let intento = login("laura@mail.com", "abcd1234"); // Call as module-level function
    match intento {
        Some(u) => println!("Login exitoso: {} - {}", u.nombre, u.email),
        None => println!("Email o password incorrectos"),
    }

    // Agregar un nuevo usuario
    let nuevo = crear_usuario(
        &mut usuarios,
        "Laura".to_string(),
        "laura@mail.com".to_string(),
        "1234".to_string(),
    );
    println!("Usuario creado: {} - {} ({})", nuevo.id, nuevo.nombre, nuevo.email);

    // Mostrar todos los usuarios
    for u in &usuarios {
        println!("{} - {} ({})", u.id, u.nombre, u.email);
    }

    // Buscar un usuario específico
    let resultado = ver_usuario(1); // Call as module-level function
    match resultado {
        Some(u) => println!("Usuario encontrado: {} - {} ({})", u.id, u.nombre, u.email),
        None => println!("Usuario no encontrado"),
    }
}

