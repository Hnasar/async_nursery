// Tested:
//
// ✔ Basic usage within function.
// ✔ A nursery passed in to a function that uses it to spawn.
//
mod common;

use common::{ import::*, DynResult };


// Basic usage within function.
//
#[ async_std::test ]
//
async fn in_method() -> DynResult
{
	let nursery = Nursery::new( AsyncStd )?;

	nursery.nurse( async { 5 + 5 } )?;
	nursery.nurse( async { 5 + 5 } )?;

	nursery.stop();

	let sum = nursery.fold( 0, |acc, x| async move { acc + x } ).await;

	assert_eq!( 20, sum );

	Ok(())
}


// A nursery passed in to a function that uses it to spawn.
//
#[ async_std::test ]
//
async fn outlive_method() -> DynResult
{
	fn outlive( nursery: &Nursery<AsyncStd, usize> ) -> DynResult
	{
		nursery.nurse( async { 5 + 5 } )?;
		nursery.nurse( async { 5 + 5 } )?;

		Ok(())
	}

	let nursery = Nursery::new( AsyncStd )?;

	outlive( &nursery )?;
	nursery.stop();

	let sum = nursery.fold( 0, |acc, x| async move { acc + x } ).await;

	assert_eq!( 20, sum );

	Ok(())
}