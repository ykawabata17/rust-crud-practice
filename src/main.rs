use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;

use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::sync::Mutex;


// ###############################################
// ここからはデータ構造を定義していくフェーズ

// Ticket構造体を定義
// データをシリアライズ/デシリアライズするためにderiveする
#[derive(Serialize, Deserialize)]
struct Ticket {
    id: u32,
    author: String,
}

// カスタムエラー構造体を定義
#[derive(Debug, Serialize)]
struct ErrNold {
    id: u32,
    err: String,
}

// Ticket型のResponderトレイトを実装する
impl Responder for Ticket {
    type Body = BoxBody;        // これ以降BoxBody型をBody型と言い換えることにする

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // serde_jsonのto_stringでjsonにシリアル化する
        let res_body = serde_json::to_string(&self).unwrap();

        // HttpResponseを作り，Contentタイプをセットする
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

// ErrNold型のResponseErrorトレイトを実装する
impl ResponseError for ErrNold {
    // ステータスコードを返す
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND       // status code 404
    }

    // エラーの内容を本文から
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        let res = HttpResponse::new(self.status_code());
        res.set_body(BoxBody::new(body))
    }
}

// RespoonseErrorにはトレイト境界が必要
// #[derive(Debug, Serialize)] でDebugのトレイト境界は満たされるがDisplayは満たされない
// ErrNold型のDisplayトレイトを実装する
impl Display for ErrNold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// 
struct AppState {
    tickets: Mutex<Vec<Ticket>>,
}

// ###############################################
// ここからはルートハンドラーを作っていくフェーズ

// チケットを作成する関数の定義
#[post("/tickets")]
async fn post_ticket(req: web::Json<Ticket>, data: web::Data<AppState>) -> impl Responder {
    let new_ticket = Ticket {
        id: req.id,
        author: String::from(&req.author),
    };

    let mut tickets = data.tickets.lock().unwrap();

    let response = serde_json::to_string(&new_ticket).unwrap();

    tickets.push(new_ticket);
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

// チケット全て取得する関数の定義
#[get("/tickets")]
async fn get_tickets(data: web::Data<AppState>) -> impl Responder {
    let tickets = data.tickets.lock().unwrap();

    let response = serde_json::to_string(&(*tickets)).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

// チケットをID指定で取得する関数の定義
#[get("/tickets/{id}")]
async fn get_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Ticket, ErrNold> {
    let ticket_id: u32 = *id;
    let tickets = data.tickets.lock().unwrap();

    // チケットIDと一致するチケットをベクターで検索
    let ticket: Vec<_> = tickets.iter()
                            .filter(|x| x.id == ticket_id)
                            .collect();
    
    if !ticket.is_empty() {
        Ok(Ticket {
            id: ticket[0].id,
            author: String::from(&ticket[0].author)
        })
    } else {
        let response = ErrNold {
            id: ticket_id,
            err: String::from("ticket not found")
        };
        Err(response)
    }
}

// チケットの情報をIDから書き換える関数の定義
#[put("/tickets/{id}")]
async fn update_ticket(id: web::Path<u32>, req: web::Json<Ticket>, data: web::Data<AppState>) -> Result<HttpResponse, ErrNold> {
    let ticket_id: u32 = *id;

    // 新しいチケットを作成
    let new_ticket = Ticket {
        id: req.id,
        author: String::from(&req.author),
    };

    let mut tickets = data.tickets.lock().unwrap();

    let id_index = tickets.iter()
                    .position(|x| x.id == ticket_id);

    match id_index {
        Some(id) => {
            let response = serde_json::to_string(&new_ticket).unwrap();
            tickets[id] = new_ticket;       // ここで更新作業が行われる
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(response)
            )
        },
        None => {
            let response = ErrNold {
                id: ticket_id,
                err: String::from("ticket not found")
            };
            Err(response)
        }
    }
}

// チケットIDからそのチケットを削除する関数の定義
#[delete("/tickets/{id}")]
async fn delete_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Ticket, ErrNold> {
    let ticket_id: u32 = *id;
    let mut tickets = data.tickets.lock().unwrap();

    let id_index = tickets.iter()
                    .position(|x| x.id == ticket_id);

    match id_index {
        Some(id) => {
            let deleted_ticket = tickets.remove(id);
            Ok(deleted_ticket)
        },
        None => {
            let response = ErrNold {
                id: ticket_id,
                err: String::from("ticket not found")
            };
            Err(response)
        }
    }
}

// ###############################################
// ここからはサーバーを作っていくフェーズ
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let app_state = web::Data::new(AppState{
        tickets: Mutex::new(vec![
            Ticket {
                id: 1,
                author: String::from("ReLU")
            },
            Ticket {
                id: 2,
                author: String::from("Bob")
            },
        ])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(post_ticket)
            .service(get_ticket)
            .service(get_tickets)
            .service(update_ticket)
            .service(delete_ticket)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}