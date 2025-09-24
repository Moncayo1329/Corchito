mod routes; // importa la carpeta routes, le dice a rust que hay un modulo llamado routes que contiene otros archivos.
use routes::usuario;

fn main(){
    println!("Servidor corchito iniciando...");


    // creamos un vector mutable para usuarios 

    let mut usuarios = usuario::listar_usuarios();

    // Agregar un nuevo usuario
    let nuevo = usuario::crear_usuario(
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

    // 3️⃣ Mostrar todos los usuarios
    for u in &usuarios {
        println!("{} - {} ({})", u.id, u.nombre, u.email);
    }

    // 4️⃣ Buscar un usuario específico
    let resultado = usuario::ver_usuario(1);

    match resultado {
        Some(u) => println!("Usuario encontrado: {} - {} ({})", u.id, u.nombre, u.email),
        None => println!("Usuario no encontrado"),
    }
}

    // simulamos llamadas a rutas 
    //routes::usuario::listar_usuarios(); // llamado a la funcion que definimos en usuario.rs
    //routes::usuario::ver_usuario(1);


