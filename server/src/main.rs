#![deny(warnings)]

use warp::{Filter, Future, Stream};


fn main()
{
	pretty_env_logger::init();


	let routes = warp::path( "echo" )

		.and( warp::ws2() ) // The `ws2()` filter will prepare the Websocket handshake.

		.map( |ws: warp::ws::Ws2|
		{
			// And then our closure will be called when it completes...
			//
			ws.on_upgrade( |websocket|
			{
				// Just echo all messages back...
				//
				let ( tx, rx ) = websocket.split();

				rx.forward( tx ).map(|_| ()).map_err( |e|
				{
					eprintln!( "websocket error: {:?}", e );
				})
			})
		})
	;


	println!( "websocket: listening on localhost:3030" );

	warp::serve( routes ).run(( [127, 0, 0, 1], 3030 ));
}


