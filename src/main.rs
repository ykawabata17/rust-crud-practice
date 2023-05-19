use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;

use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::sync::Mutex;


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


fn main() {
    println!("Hello, World!");
}