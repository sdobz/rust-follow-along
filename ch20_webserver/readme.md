# Chapter 20 - Webserver

Starting single threaded
incoming iterates over connection ATTEMPTS

Write a new function to handle reading request
Alloc buffer `let mut buffer = [0; 512];`
512 is enough for our purposes
Read into buffer, print
`from_utf8_lossy` replaces invalid unicode with `U+FFFD REPLACEMENT CHARACTER`

Have a get method, 404 method, sleep method

## Using threads

Create a fixed size thread pool to prevent DoS issues
Look up the spawn method of Thread to see what signature we need

 - FnOnce() - needs () because type includes parameters, omit return value
 - Send - Has to send to another thread
 - 'static - Don't know how long thread will last

Create a Vec of thread handles and start them up when creating the thread pool

## Sending work to threads

Create a Worker abstraction that distributes work to thread

Use multiple producer single consumer channel, store sender in thread pool give receiver to thread
Have to wrap receiver in a mutex and an arc to pass into for loop/worker

Awful box ownership hack, please ignore

## Terminating threads

When the ThreadPool is Drop'ped it joins all of the threads
The item going over the channel is changed to Message enum with a NewJob variant or a Terminate
Worker loop breaks on Terminate

Send Terminate down line 4 times
Join on all threads

Server stopped gracefully