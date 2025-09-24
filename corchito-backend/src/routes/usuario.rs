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

pub fn ver_usuario(id: u32) -> Option<Usuario>{

let usuarios = listar_usuarios();
usuarios.into_iter().find(|u| u.id == id)
}

