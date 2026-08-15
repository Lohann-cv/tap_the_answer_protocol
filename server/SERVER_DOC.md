# THE SERVER

## Tech Stack Used

> Tokio

## Basic rust task management

**.await** => We're waiting for the task to end.

With basic threading :

```rust
#[tokio::main]
async fn main() {
	greet("Jean").await; // 1sec
	greet("Robert").await; // 1sec
	greet("Moulinette").await; // 1sec
	// 3 sec
}
```

With ```tokio::join!``` :

```rust
#[tokio::main]
async fn main() {
	// the 3 function are being executed at the same time
	tokio::join!(
	    greet("Jean"),
	    greet("Robert"),
	    greet("Moulinette")
    ).await;
	// 1 sec
}
```

For many task use ```tokio::spawn()``` :

```rust
#[tokio::main]
async fn main() {
	let mut handles = vec![];
    // we're launching 1000 task without blocking anything
	for i in 0..1000 {
		let handle = tokio::spawn(async move {
			greet(&format!("Name {}", i)).await;
		});
		// we're putting each joinhandle in the vector
		handles.push(handle);
	}

	// and waiting for each to finish
	for handle in handles {
		handle.await.unwrap();
	}
}
```

When we want to wait for tasks to finish we use ```tokio::select!``` :

```rust
async fn task_waiter() {
	let task1 = doing_things();
	let task2 = doing_other_things();

	tokio::select! {
		_ = task1 => println!("Task 1 done"),
		_ = task2 => println!("Task 2 done"),
	}
}
```

When we want task to communicates we use ```tokio::sync::mpsc::channel()``` :

```rust
#[tokio::main]
async fn main() {
	// we're creating a channel
	let (tx, mut rx) = tokio::sync::mpsc::channel(100);

	// Messages sending
	tokio::spawn(async move {
		for i in 0..5 {
			println!("Sending: {}", i);
			tx.send(i).await.ok();
			tokio::time::sleep(Duration::from_secs(1)).await;
		}
	});

	tokio::spawn(async move {
		while let Some(msg) = rx.recv().await {
			println!("Reciev: {}", msg);
		}
	}).await.ok();
}
```

When we need everyone to have one message we use broadcast

```rust
#[tokio::main]
async fn main() {
	// we're creating the broadcast channel
	let (tx, _rx) = tokio::sync::broadcast::channel(100);

	// let's say we have 3 client
	for client_id in 0..3 {
		let mut rx = tx.subscribe();
		tokio::spawn(async move {
			while let Ok(msg) = rx.recv().await {
				println!("Client {} reciev: {}", client_id, msg);
			}
		});
	}

	// we then send a message to everyone
	tokio::time::sleep(Duration::from_secs(1)).await;
	tx.send("Salut tout le monde!").ok();
	tokio::time::sleep(Duration::from_secs(1)).await;
}
```

And when we must share some data we use mutex :

```rust
#[tokio::main]
async fn main() {
	let counter = Arc::new(tokio::sync::Mutex::new(0));

	let mut handles = vec![];

	for i in 0..5 {
		let counter - Arc::clone(&counter);

		let handle = tokio::spawn(async move {
			let mut count = counter.lock().await;
			*count += 1;
			println!("Task {} increment the counter to {}", i, *count);
			// it's unlocked when out of scope
		});
		handles.push(handle);
	}

	for handle i handles {
		handle.await.ok();
	}

    let final_count = counter.lock().await;
    println!("Final counter: {}", *final_count);
}
```
