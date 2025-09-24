mod routes; // importa la carpeta routes, le dice a rust que hay un modulo llamado routes que contiene otros archivos.
use routes::usuario;

fn main(){
    println!("Servidor corchito iniciando...");



    // 1. Probar listar usuarios.

    let usuarios = usuario::listar_usuarios();
    println!("Lista de usuarios:");
    for u in &usuarios {

        println!("{} - {} ({})", u.id, u.nombre, u.email);
    }

    // 2. Probar ver un usuario especifico

    let resultado = usuario::ver_usuario(1);

    match resultado {
        Some(u) => {

            println!("Usuario encontrado: {} - {} ({})", u.id, u.nombre, u.email)
        }
    
    None => {
        println!("Usuario no encontrado")
    }
}

    // simulamos llamadas a rutas 
    //routes::usuario::listar_usuarios(); // llamado a la funcion que definimos en usuario.rs
    //routes::usuario::ver_usuario(1);
}

