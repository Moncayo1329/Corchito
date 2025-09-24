mod routes; // importa la carpeta routes, le dice a rust que hay un modulo llamado routes que contiene otros archivos.

fn main(){
    println!("Servidor corchito iniciando...")


    // simulamos llamadas a rutas 
    routes::usuario::listar_usuarios(); // llamado a la funcion que definimos en usuario.rs
    routes::usuario::ver_usuario(1);
}

