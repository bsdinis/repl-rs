use counter_client_protocol::counter_client::CounterClient;
use eyre::Result;
use structopt::StructOpt;
use tonic::transport::{Channel, Uri};

#[structopt(name = "clnt-state-machine")]
#[derive(StructOpt)]
struct Opt {
    /// location of the server
    #[structopt(name = "server", long)]
    server_addr: Uri,

    /// operation
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Status,
    Get,
    Incr { step: i64 },
    Decr { step: i64 },
    AtomicIncr { before: i64, step: i64 },
    AtomicDecr { before: i64, step: i64 },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();

    let remote = CounterClient::new(Channel::builder(opt.server_addr).connect_lazy()?);

    match opt.cmd {
        Command::Status => do_status(remote).await,
        Command::Get => do_get(remote).await,
        Command::Incr { step } => do_incr(remote, step).await,
        Command::Decr { step } => do_decr(remote, step).await,
        Command::AtomicIncr { before, step } => do_atomic_incr(remote, before, step).await,
        Command::AtomicDecr { before, step } => do_atomic_decr(remote, before, step).await,
    }
}

async fn do_get(mut rem: CounterClient<Channel>) -> Result<()> {
    println!("executing get()");
    let res = rem
        .get(counter_client_protocol::GetCounterReq {})
        .await?
        .get_ref()
        .cur;
    println!("counter value = {}", res);
    Ok(())
}

async fn do_status(mut rem: CounterClient<Channel>) -> Result<()> {
    println!("executing status()");
    println!(
        "status of replica: {:?}",
        rem.status(counter_client_protocol::StatusReq {})
            .await?
            .get_ref()
    );
    Ok(())
}

async fn do_incr(mut rem: CounterClient<Channel>, step: i64) -> Result<()> {
    println!("executing incr({})", step);
    let res = rem
        .incr(counter_client_protocol::IncrCounterReq { step })
        .await?
        .get_ref()
        .cur;
    println!("counter value = {}", res);
    Ok(())
}

async fn do_decr(mut rem: CounterClient<Channel>, step: i64) -> Result<()> {
    println!("executing decr({})", step);
    let res = rem
        .decr(counter_client_protocol::DecrCounterReq { step })
        .await?
        .get_ref()
        .cur;
    println!("counter value = {}", res);
    Ok(())
}

async fn do_atomic_incr(mut rem: CounterClient<Channel>, before: i64, step: i64) -> Result<()> {
    println!(
        "executing atomic_incr(before = {}, step = {})",
        before, step
    );
    match rem
        .atomic_incr(counter_client_protocol::AtomicIncrCounterReq { before, step })
        .await?
        .get_ref()
    {
        counter_client_protocol::AtomicIncrCounterResp { cur, success: true } => {
            println!("counter value = {} [incremented successfully]", cur)
        }
        counter_client_protocol::AtomicIncrCounterResp {
            cur,
            success: false,
        } => println!("counter value = {} [failed to increment]", cur),
    };
    Ok(())
}

async fn do_atomic_decr(mut rem: CounterClient<Channel>, before: i64, step: i64) -> Result<()> {
    println!(
        "executing atomic_decr(before = {}, step = {})",
        before, step
    );
    match rem
        .atomic_decr(counter_client_protocol::AtomicDecrCounterReq { before, step })
        .await?
        .get_ref()
    {
        counter_client_protocol::AtomicDecrCounterResp { cur, success: true } => {
            println!("counter value = {} [decremented successfully]", cur)
        }
        counter_client_protocol::AtomicDecrCounterResp {
            cur,
            success: false,
        } => println!("counter value = {} [failed to decrement]", cur),
    };
    Ok(())
}
