#[derive(Debug, Clone)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    pub password: String,
}

pub fn listar_usuarios() -> Vec<Usuario> {
    // Placeholder: Replace with actual logic (e.g., database query)
    vec![
        Usuario {
            id: 1,
            nombre: "Laura".to_string(),
            email: "laura@mail.com".to_string(),
            password: "abcd1234".to_string(),
        },
        Usuario {
            id: 2,
            nombre: "Juan".to_string(),
            email: "juan@mail.com".to_string(),
            password: "1234".to_string(),
        },
    ]
}

pub fn login(email: &str, password: &str) -> Option<Usuario> {
    let usuarios = listar_usuarios();
    usuarios
        .into_iter()
        .find(|u| u.email == email && u.password == password)
}

pub fn crear_usuario(
    usuarios: &mut Vec<Usuario>,
    nombre: String,
    email: String,
    password: String,
) -> Usuario {
    let id = usuarios.len() as i32 + 1; // Simple ID generation
    let nuevo = Usuario {
        id,
        nombre,
        email,
        password,
    };
    usuarios.push(nuevo.clone());
    nuevo
}

pub fn ver_usuario(id: i32) -> Option<Usuario> {
    let usuarios = listar_usuarios();
    usuarios.into_iter().find(|u| u.id == id)
}