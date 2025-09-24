// funciones que manejarian cada endpoint 

// Definir el struct del usuario. 

#[derive(Debug, Clone)] // Debug: permite imprimir, Clone: permite duplicar

pub struct Usuario { 
    pub id: u32,
    pub nombre: String,
    pub email: String,
    pub password: String, // por ahora en texto plano, luego se puede hashear. 

}

// significa que son funciones publicas, para poder llamarlas desde el main.rs
// id;u32 simulamos un parametro que vendria de la url.

// paso 2 crear un vector de usuarios de prueba y devolver resultados reales en vez de solo printil!

pub fn listar_usuarios() -> Vec<Usuario> {
    vec![
  Usuario{ 
id:1, 
nombre:"Michael".to_string(),
email: "moncayomichael23@gmail.com".to_string(),
password:"1234".to_string(),
  },

  Usuario {id:2, 
 nombre:"Ana".to_string(),
email:"anabel@gmail.com".to_string(),
password:"abdb".to_string(),
},
    ]
}

// Funcion para buscar usuario por id

pub fn ver_usuario(id: u32) -> Option<Usuario>{

let usuarios = listar_usuarios();
usuarios.into_iter().find(|u| u.id == id)
}



// Funcion para crear usuario nuevo. 

pub fn crear_usuario(
usuarios: &mut Vec<Usuario>,
nombre:String,
email:String,
password:String,
) -> Usuario {
    let nuevo_id = usuarios.iter().map(|u| u.id).max().unwrap_or(0) + 1;


let nuevo_usuario = Usuario {
    id: nuevo_id,
    nombre,
    email,
    password,

}; 

usuarios.push(nuevo_usuario.clone()); // agregamos al vector 


nuevo_usuario // devolvemos el usuario creado.

}