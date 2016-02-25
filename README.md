# Web Test

A lot of web devs like to use Go nowadays for building HTTP backed APIs. Go is
pretty easy to pick up and has really nice performance characteristics so it's
no surprise that a lot of people like it. I've had the pleasure of using the
language myself for various projects but never that extensively.

Another language that has been peeking my interest lately is Rust. Rust is also
VERY fast, but isn't normally thought of as a "web language". Rust gives
you the same amount of control that C or C++ gives you so a lot of people have
been using it in other domains like operating systems and emulators.

Being primarily a web developer at my job, I thought it would be interesting to
see how Rust was doing on the web development front. Since I'd expect it to
have very good performance I thought I'd see how Rust performed compared to Go.

I hope to write this up as a proper blog post in the future.

## The Test

I built a web sever that simply proxied for Redis, fetching the value for a
given key. I built the web app in Go using the built-in http package and since
Rust doesn't have built-in http support, I tried out two libraries: Nickel and
Iron.

I then ran the following command to test load against these servers:

`wrk -d10 -c20 -t10 "http://localhost:8000/get?key=foo"`

This command hits the endpoint for 10 seconds with 20 concurrent connections on
10 threads. It then reports back its findings.

## Performance Results

None of these numbers should be taken literally as they represent a sample size of 1. They should, however, offer an idea of relative perforamce of one language-framework to another.

Rust Iron:

```
10 threads and 20 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.52ms  373.03us   8.45ms   90.54%
    Req/Sec     1.33k   106.32     1.96k    71.90%
  133070 requests in 10.10s, 13.20MB read
Requests/sec:  13171.73
Transfer/sec:      1.31MB
```

Rust Nickel:

```
10 threads and 20 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   355.22us  105.97us   4.55ms   94.03%
    Req/Sec     4.67k     1.36k    6.08k    65.35%
  140846 requests in 10.10s, 18.54MB read
Requests/sec:  13946.56
Transfer/sec:      1.84MB
```

Go:

```
10 threads and 20 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.08ms    4.01ms 150.54ms   98.73%
    Req/Sec     2.59k   274.96     3.96k    73.39%
  259153 requests in 10.10s, 29.41MB read
Requests/sec:  25653.38
Transfer/sec:      2.91MB
```

## Impressions

### Latency

First impressions are that both languages are quite fast. Nickel had the most
impressive latency record usually clocking in at well under 400 microseconds on
average per request. Go had a pretty good latency track record, consistently
beating Iron but always losing to Nickel. Go did however have spikes in latency
very regularly, and if the number of connections went too high, it would error out
while both Rust frameworks remained steady.

If latency is your concern, Rust seems to be something to watch out for, though
Go is not too shabby. I imagine both languages will improve here, but Rust is
probably going to always beat Go in terms of raw speed for various reasons, GC
only being one of them.

### Throughput

While both Rust frameworks had very nice figures to show in terms of
throughput, Go signficantly out shone the crowd. This makes sense as Go's
concurrency story is very nice. I'm not down on Rust here, however, as this will
continue to get better since higher level concurrency paradigms are something
the Rust core team has left to the community. I imagine that Rust http libraries
will one day be significantly better in this area than they are today. Whether
they can reach the levels of Go is something that remains to be seen. The good
thing is that reach such a level isn't a technical constraint.

## Developer Experience

I'd like to write more about developer experience and some other non-performace
related topics that came out of this experiment, but I will leave those another
commit.
