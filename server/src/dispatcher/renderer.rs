use proto::{
    build_engine, evaluate_frame, load_script,
    renderer_server::{Renderer, RendererServer},
    stream_frame,
};
use tonic::{async_trait, Request, Response, Status, Streaming};

mod proto {
    use tonic::include_proto;

    include_proto!("renderer");
}

pub struct RendererImpl;

#[async_trait]
impl Renderer for RendererImpl {
    type StreamFrameStream = Streaming<stream_frame::Result>;

    async fn load_script(
        &self,
        req: Request<load_script::Request>,
    ) -> Result<Response<load_script::Result>, Status> {
        todo!()
    }

    async fn build_engine(
        &self,
        req: Request<build_engine::Request>,
    ) -> Result<Response<build_engine::Result>, Status> {
        todo!()
    }

    async fn evaluate_frame(
        &self,
        req: Request<evaluate_frame::Request>,
    ) -> Result<Response<evaluate_frame::Result>, Status> {
        todo!()
    }

    async fn stream_frame(
        &self,
        req: Request<Streaming<stream_frame::Request>>,
    ) -> Result<Response<Self::StreamFrameStream>, Status> {
        todo!()
    }
}

impl RendererImpl {
    pub fn service() -> RendererServer<RendererImpl> {
        RendererServer::new(RendererImpl)
    }
}
