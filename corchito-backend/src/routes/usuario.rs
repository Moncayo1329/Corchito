// funciones que manejarian cada endpoint 

// Definir el struct del usuario. 

pub struct Usuario { 
    pub id: u32,
    pub nombre: String,
    pub email: String,
    pub password: String, // por ahora en texto plano, luego se puede hashear. 

}

// significa que son funciones publicas, para poder llamarlas desde el main.rs
// id;u32 simulamos un parametro que vendria de la url.

pub fn listar_usuarios() {
    println!("Esta funcion listaria todos los nuevos usuarios");
}

pub fn ver_usuario(id: u32){

    println!("Esta funcion mostraria el usuario com id:{}", id);

}

