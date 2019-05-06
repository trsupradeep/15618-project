# Comparison of Multi-threading between C++ and Rust (OpenMP vs Rayon)

## Summary
We implemented and compared four benchmarks in Rust and C++ using Rayon and OpenMP respectively. To provide in-depth comparison, we have used multiple configurations for each benchmark. Rayon performed as good as OpenMP in cases where the underlying algorithm or compiler gave an advantage or edge. The downfalls of Rayon are the under-optimized computing function, cost of creating splits of work, and stealing when compared to a possible static scheduling. Rayon performed better for sorting and multiplication of larger matrices. In all other benchmarks, OpenMP had the upper hand. Another advantage Rayon possessed was the failure in compilation of code that had unsafe sharing of variables between threads, allowing us to write correct code always. 

## Motivation
Over the course of 15-618, we have learned that in order to scale the performance of a software with the hardware, we as developers need to write concurrent, fast and correct applications. While decomposing a given algorithm into parallel workloads, developers need to be careful about new kinds of error including deadlock, livelock and data-races. Hence, in order to get improved performance through parallelism, we need to understand the nuances of these errors and anticipate them in advance.

What Rust believes to provide is an abstraction for thread-safety with zero-cost, thus becoming highly popular among industries and developers (Mozilla being the strongest influence). Rust enables safe concurrency with data locks and message-based communication channels. Furthermore, Rust performs compile time analysis on threads data behavior to determine potential problems. Rust’s ownership construct and concurrency rules offer powerful compile time tool to help programmers write safe and efficient concurrent programs. Through this project, we want to get a deeper understanding of how Rust solves the issue of data-races and what speed it provides against C++, as it promises to solve many of the issues a programmer faces when parallelizing C++ code, at zero-cost. Rust provides various libraries (called as crates) for multithreading abstraction. Rayon and Crossbeam are two of the most popular ones, as suggested by the Rust community. 

## Background
### What is Rust?
Rust is new coding language rising the ranks, aiming at safe concurrency, data safe programming at zero abstraction cost. Rust performs compile time analysis on threads data behavior to determine potential problems. Rust’s ownership construct and concurrency rules offer powerful compile time tool to help programmers write safe and efficient concurrent programs. This resolves the classic problems of using a variable that is shared between threads without mutual exclusion. At the same time, it keeps you away from most segmentation fault. But it cannot save you from deadlocks, or poorly written code. There is a ‘Rust’ way of writing code which must be followed to write the most optimized code that will provide the same low-level code as C/C++. Such zero-cost abstraction is highly beneficial for developers, but it comes with the cost of fighting the compiler to get a compiled code (mainly the borrow checker). Also, Rust gives a guarantee that a code which compiles, will almost work perfectly.

### Rayon
Rayon is a data-parallelism library for Rust. It is extremely lightweight and makes it easy to convert a sequential computation into a parallel one. It also guarantees data-race freedom. It provides an abstraction for data parallelism which is really simple and easy to implement. For example, if you write a serial code with an iterator, you can simply turn it parallel by using parallel Iterator method. 


### How Rayon works?
Rayon uses the technique of “work stealing” that is very similar to what is employed by the Cilk abstraction for C/C++, hence very suitable for “divide and conquer” type of workload. Rayon when compared to Cilk is much easier to use and is being actively maintained by the developer community. 

The basic idea is that, on each call to `join(a, b)`, we have identified two tasks `a` and `b` that could safely run in parallel, but we don’t know yet whether there are idle threads. All that the current thread does is to add `b` into `a` local queue of “pending work” and then go and immediately start executing `a`. Meanwhile, there is a pool of other active threads (typically one per CPU, or something like that). Whenever it is idle, each thread goes off to scour the “pending work” queues of other threads: if they find an item there, then they will steal it and execute it themselves. So, in this case, while the first thread is busy executing `a`, another thread might come along and start executing `b`.

Once the first thread finishes with a, it then checks: did somebody else start executing b already? If not, we can execute it ourselves. If so, we should wait for them to finish but while we wait, we can go off and steal from other processors, and thus try to help drive the overall process towards completion.

Inherently, this process of stealing and coherently working adds dynamic balancing of load between the threads but adds slight overhead. These behaviors have been analyzed using benchmarks which are provided in the Result section. 

### What Rayon provides as features?
There are two ways of using Rayon:
1.	High-Level parallel constructs: one of the most efficient way of parallelizing in Rayon.
    *	`par_iter()`: An abstraction over the join method of Rayon, which allows you to iterate similar to iter() method, with all the other abstracting functions like map(), for_each(), sum(), fold() etc. 

    *	`par_sort()`: A parallel sorting abstraction, which works similar to a sort() trait, but has parallelism using Rayon. Rayon provides multiple sorting abstraction that allow sort by keys. This helps in using this API for different sorting situation. par_sort_by()

    *	`par_extend()`: This can be used to efficiently grow collections with items produced by a parallel iterator. 

2.	Custom tasks: It lets you divide your work into parallel tasks yourself.
    *	`join()`: This method is similar to spawning two threads, one executing each of the two closures. You can use it to split a job into two smaller jobs. This join works on stealing style, so it incurs lower overhead than a simple spawn of two threads. 

    *	`scope()`: This method creates a scope within which you can create any number of parallel tasks. We can perform any kind of parallel task using this, but they recommend using join(), as this is not as optimized. Though we can say that a static assignment will be faster. 

    *	`ThreadPoolBuilder()`: It can be used to create your own thread pools or customize the global one. This can be used to modify number of threads. This can be used to create a thread closure while spawning threads. 

### OpenMP
Through the coursework, we have gained a good understanding of OpenMP abstraction for parallelism in C/C++ which allows compiler directives to be embedded into a serial code. Its API significantly simplifies writing multi-threaded programs in Fortran, C and C++. Although the ease of use and flexibility are the main advantages of OpenMP, it is up to the developer to write safe code. If one is not careful, they could easily run into the hazards of data-races and deadlocks. This could potentially limit the scalability of the code (one of the reasons why it is not employed in modern-day browsers). 

### How does OpenMP work?
OpenMP uses the fork-join execution model i.e. the master thread spawns a team of threads as needed to allow multiple threads of execution perform tasks. Threads are created in OpenMP using the parallel construct. The parallel construct itself creates a “Single Program Multiple Data” program. Although it ensures that computations are performed in parallel it does not distribute the work among the threads in a team. If the user does not specify any worksharing, the work will be replicated. On the other hand, the worksharing construct of OpenMP allows the user to split up pathways through the code between these threads. For example, #pragma omp for distributes iterations over threads. Scheduling of these iterations can be determined by static, dynamic, guided and runtime. All threads synchronize at an implicit barrier at the end of the loop unless nowait clause is specified. However, the programmer needs to use synchronization constructs to impose order constraints and to protect access to shared data.

### Comparison of Rayon and OpenMP
Parallelism is hard is get right in most programming environments because normally it involves substantial refactoring of code. With Rust and its multithreading crates like Rayon, a programmer with little to no knowledge of parallelism can simply convert the sequential iterations of their code to parallel iterations by importing the crate and changing `iter` to `par_iter`. If its thread-unsafe to do so, the Rust code simply won’t compile. This makes Rust code much more scalable than C++ and this is the main difference between Rust and C++. Besides, Rayon’s work-stealing inherently divides the load almost equally amongst the threads providing good performance for imbalanced workload. However, since the Rayon crate is still in its nascency, it does not support auto-vectorization of the code unlike OpenMP which has directives (#pragma simd) to do so. In rust, we would have to import additional crates like faster to support forced vectorization of code.


## Approach

To compare two different parallelism libraries of different languages, we need to see its practicality and see how they perform for the common uses. A developer will look for three things in such a library, ease of use, safe concurrency and speed. Here we choose to benchmark the Rayon library against simpler but fundamental benchmarks and try to see patterns in understanding and further assess it with deeper code research on the Github repository of the crate.

 
### Benchmarks
Benchmarks for the comparison were chosen with few objectives in mind. We wanted to see the performance of the libraries for the inbuilt provided methods that compete directly. Apart from that, we wanted to how they handle dynamic load when provided with such a problem. Rayon inherently can handle dynamic load, but OpenMP needs the schedule(dynamic) directive to provide dynamic balance.
 
Benchmarks chosen:
1.	Mandelbrot – It is a simple and embarrassingly parallel benchmark that generates images for visualization of a famous set of complex numbers called the Mandelbrot set. The benchmark allows you to vary the size of the generated image and the maximum number of iterations per point. Reason for use: This is an imbalanced workload since the work required for each pixel of the image varies. For this benchmark, it would be interesting to analyze how parallelism libraries schedule the work efficiently amongst threads. Because of the inherent dynamic scheduling feature of Rayon, we expect it to perform better than OpenMP.
 
2.	Matrix Multiplication – A commonly used operation in the recent times, it is a simple benchmark which tests for task division for multiple threads. Reason for use: This is a compute bound problem; thus, we can see how the parallelism libraries behave when doing this task. We also can see how these algorithms behave when written in a cache friendly manner or multi-thread advantageous manner. Another comparison can be done on how both handle a job division when provided with the actual job division (i.e. row first dot products).
 
3.	Unstable-Stable Sorting – We are comparing the stable (merge-sort) and unstable (quicksort) sort functionality of Rayon and GNU-parallel sort provided as an extension of OpenMP.  In this benchmark, we have sorted 1M, 10M, and 100M elements ascendingly from randomly generated values. Reason for use: The divide and conquer nature of these sorting algorithms makes it a good benchmark for parallelization. Rayon could be more suitable because of “work-stealing” abstractions. 

4.	Reduction – OpenMP added a reduction feature which uses a reduction primitive `reduction(+:result_sum)`. This was specifically added to perform functions like a dot product which is used in Linear Algebra and Geometry. Not only that, but any kind of reduction can be done, where we add result of some operation on a single element of the array and accumulate it. Reason for use: Though the problem may be memory bound, this is a common operation in Physics (or even in Matrix multiplication), whose speed will help achieve higher speed ups for complex problems. This can also give us insights on how Rayon behaves when given a load that can actually be divided statically. 
 
 
### Use the best serial code for Rust + Rayon
The first part of every benchmark is to write the most optimized serial code in both. The reason for this is that, an unoptimized code if used with Rayon and OpenMP will lead to bad parallelism while scaling to cores for such simple benchmarks. Hence, we must either observe the assembly code of the compiled executable or try to see which written code provides the best optimized assembly. This precaution is taken because both libraries provide a simple abstraction to parallelize by adding a few lines or a few method changes. An unoptimized code, will result in bad parallelism this way. 
 
For example, in one of our benchmarks, Mandelbrot, we used a crate named num, which provides a wrapper for complex number variables (`num::Complex32`, `num::Complex64`). Though it is mentioned that this code should perform as well as simple serial, it has some extra assembly commands added to its compiled executable. We have a more detailed analysis for this in the result section. 
 
In almost each benchmark we observed the serial performance of Rust code and tried to get it close to C++. We did succeed in almost all cases. Due to this, we have a fair comparison. The way the libraries perform the low-level work will be done the same way in both libraries and the only difference that is observed will be from the framework. 

 
### Re-optimization
Rayon provides a very simple way to create parallelism, but as mentioned before, different ways of writing the same code will give us different speeds due to the way the compiler optimizes it. Though this is applicable to OpenMP too, we have the option to eliminate most issues by using `schedule(static)` on the correct For loop, to decide the static division of work between the threads.
 
The reason we added this as a part of our design to test is because we faced this issue in both Mandelbrot and Matrix multiplication. Naturally we believed that if we write a working code for the serial case, we can easily parallelize it using the abstractions in each library. But in the case of Mandelbrot, we found that writing parallel code over pixels for Rust/Rayon gave a better speedup when compared writing parallel code over rows. This was contradictory to our understanding of the benchmark in C++ and Assignment 1 from 15-618 course. Nevertheless, such issues were observed, documented, and understood, which gave us interesting insights for Rayon. 

During Matrix Multiplication, we were able to write multiple variations of the code, which allowed for a different style of parallelization. Only through speed-up graphs, we were able to analyze the difference in speed. Also, to eliminate the effect of cache miss, only for testing this speed we used smaller matrices. 


