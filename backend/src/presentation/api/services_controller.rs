use actix_web::{HttpResponse, web};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct ServiceDto {
    id: String,
    business_name: String,
    description: String,
    services_provided: Vec<String>,
    rating: f64,
    reviews: u32,
    photos: Vec<String>,
    videos: Vec<String>,
    location: String,
    operating_hours: serde_json::Value,
}

/// GET /api/v1/services?q=&category=
pub async fn list_services(query: web::Query<ServiceQuery>) -> HttpResponse {
    // TODO: replace with real DB query via use case
    let all = mock_services();

    let filtered: Vec<&ServiceDto> = all
        .iter()
        .filter(|svc| {
            let matches_q = query.q.as_ref().map_or(true, |q| {
                let q_lower = q.to_lowercase();
                svc.business_name.to_lowercase().contains(&q_lower)
                    || svc
                        .services_provided
                        .iter()
                        .any(|s| s.to_lowercase().contains(&q_lower))
            });
            let matches_cat = query.category.as_ref().map_or(true, |cat| {
                cat == "Todos"
                    || svc
                        .services_provided
                        .iter()
                        .any(|s| s.to_lowercase().contains(&cat.to_lowercase()))
            });
            matches_q && matches_cat
        })
        .collect();

    HttpResponse::Ok().json(json!({ "data": filtered, "total": filtered.len() }))
}

/// GET /api/v1/services/{id}
pub async fn get_service(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let all = mock_services();

    match all.into_iter().find(|s| s.id == id) {
        Some(svc) => HttpResponse::Ok().json(svc),
        None => HttpResponse::NotFound().json(json!({"error": "Serviço não encontrado"})),
    }
}

#[derive(serde::Deserialize)]
pub struct ServiceQuery {
    pub q: Option<String>,
    pub category: Option<String>,
}

fn mock_services() -> Vec<ServiceDto> {
    vec![
        ServiceDto {
            id: "019505b0-0001-7000-8000-000000000001".into(),
            business_name: "Encanamentos Silva".into(),
            description:
                "Mais de 15 anos de experiência em serviços hidráulicos residenciais e comerciais."
                    .into(),
            services_provided: vec![
                "Encanamento".into(),
                "Desentupimento".into(),
                "Caça-vazamentos".into(),
            ],
            rating: 4.7,
            reviews: 128,
            photos: vec![
                "https://placehold.co/600x400/1A73E8/white?text=Encanamentos+Silva".into(),
            ],
            videos: vec![],
            location: "São Paulo, SP".into(),
            operating_hours: json!({"segunda": {"open": "08:00", "close": "18:00"}, "terca": {"open": "08:00", "close": "18:00"}, "quarta": {"open": "08:00", "close": "18:00"}, "quinta": {"open": "08:00", "close": "18:00"}, "sexta": {"open": "08:00", "close": "18:00"}, "sabado": {"open": "08:00", "close": "14:00"}, "domingo": null}),
        },
        ServiceDto {
            id: "019505b0-0002-7000-8000-000000000002".into(),
            business_name: "Elétrica Express".into(),
            description: "Serviços elétricos rápidos e seguros para sua casa ou empresa.".into(),
            services_provided: vec![
                "Elétrica".into(),
                "Instalação Elétrica".into(),
                "Manutenção Predial".into(),
            ],
            rating: 4.5,
            reviews: 87,
            photos: vec!["https://placehold.co/600x400/34A853/white?text=Eletrica+Express".into()],
            videos: vec![],
            location: "Campinas, SP".into(),
            operating_hours: json!({"segunda": {"open": "07:00", "close": "19:00"}, "terca": {"open": "07:00", "close": "19:00"}, "quarta": {"open": "07:00", "close": "19:00"}, "quinta": {"open": "07:00", "close": "19:00"}, "sexta": {"open": "07:00", "close": "19:00"}, "sabado": {"open": "08:00", "close": "16:00"}, "domingo": null}),
        },
        ServiceDto {
            id: "019505b0-0003-7000-8000-000000000003".into(),
            business_name: "Jardins & Paisagismo".into(),
            description:
                "Transformamos espaços verdes com serviços completos de jardinagem e paisagismo."
                    .into(),
            services_provided: vec![
                "Jardinagem".into(),
                "Paisagismo".into(),
                "Poda de Árvores".into(),
            ],
            rating: 4.9,
            reviews: 53,
            photos: vec![
                "https://placehold.co/600x400/34A853/white?text=Jardins+Paisagismo".into(),
            ],
            videos: vec!["https://placehold.co/600x400/FBBC04/white?text=Video+Tour".into()],
            location: "São Paulo, SP".into(),
            operating_hours: json!({"segunda": {"open": "08:00", "close": "17:00"}, "terca": {"open": "08:00", "close": "17:00"}, "quarta": {"open": "08:00", "close": "17:00"}, "quinta": {"open": "08:00", "close": "17:00"}, "sexta": {"open": "08:00", "close": "17:00"}, "sabado": null, "domingo": null}),
        },
        ServiceDto {
            id: "019505b0-0004-7000-8000-000000000004".into(),
            business_name: "PintaTop Pinturas".into(),
            description:
                "Pinturas profissionais com acabamento impecável. Residências e comércios.".into(),
            services_provided: vec!["Pintura".into(), "Textura".into(), "Grafiato".into()],
            rating: 4.3,
            reviews: 41,
            photos: vec!["https://placehold.co/600x400/EA4335/white?text=PintaTop".into()],
            videos: vec![],
            location: "Guarulhos, SP".into(),
            operating_hours: json!({"segunda": {"open": "08:00", "close": "18:00"}, "terca": {"open": "08:00", "close": "18:00"}, "quarta": {"open": "08:00", "close": "18:00"}, "quinta": {"open": "08:00", "close": "18:00"}, "sexta": {"open": "08:00", "close": "18:00"}, "sabado": null, "domingo": null}),
        },
        ServiceDto {
            id: "019505b0-0005-7000-8000-000000000005".into(),
            business_name: "Clima Perfeito".into(),
            description:
                "Instalação, limpeza e manutenção de ar condicionado split e central com garantia."
                    .into(),
            services_provided: vec![
                "Ar Condicionado".into(),
                "Manutenção de Ar".into(),
                "Instalação Split".into(),
            ],
            rating: 4.6,
            reviews: 95,
            photos: vec!["https://placehold.co/600x400/1A73E8/white?text=Clima+Perfeito".into()],
            videos: vec![],
            location: "São Paulo, SP".into(),
            operating_hours: json!({"segunda": {"open": "08:00", "close": "20:00"}, "terca": {"open": "08:00", "close": "20:00"}, "quarta": {"open": "08:00", "close": "20:00"}, "quinta": {"open": "08:00", "close": "20:00"}, "sexta": {"open": "08:00", "close": "20:00"}, "sabado": {"open": "09:00", "close": "15:00"}, "domingo": null}),
        },
    ]
}
