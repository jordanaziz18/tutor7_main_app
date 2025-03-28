
# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
```sh
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
```

    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

- Declaring Subscriber as an interface type (or trait in Rust) in the Observer pattern guarantees extensibility and conformity to SOLID principles.  However, since there is only one type of Subscriber in view and no immediate plans to introduce any others, BambangShop will only require a single Model struct.

- DashMap's use in this instance is warranted because it gives subscribers a clear, effective map from product IDs. DashMap offers constant-time access, which enhances performance, particularly as data accumulates, in contrast to Vec, where lookups would take linear searches. It is more suited for multithreaded apps like BambangShop because it also allows safe multithreaded access. Furthermore, there is no data fragmentation when using a Singleton that provides a central Subscriber map. DashMap is the best option for performance and scalability, although a Vec would work well for small-scale use cases.

- DashMap and Singleton patterns are complementary in BambangShop, providing centralized instances and thread safety. Singleton prevents unnecessary cloning but doesn't handle concurrent access issues, while DashMap ensures safe concurrent access without manual synchronization.

#### Reflection Publisher-2

- Separating Service and Repository from the Model benefits the Single Responsibility Principle by ensuring the model represents data, the repository handles data access, and the service manages business logic. This separation enhances maintainability, modularity, testability, flexibility, and promotes a cleaner architecture with well-defined boundaries. It reduces dependencies between layers and makes the application more scalable. This approach improves code readability, simplifies maintenance, and allows development teams to work on different layers without conflicts, leading to a more efficient and structured software development process.

- Using models without separation can cause issues like bloated classes, tight coupling, and difficulty in testing. It can also lead to a cluttered and less maintainable codebase, as changes in one model can affect others, making it difficult to isolate components and rely on external services. Additionally, it violates the Single Responsibility Principle (SRP) by handling data representation, storage logic, and business rules within the same model.

- Postman is a powerful tool for testing web endpoints and ensuring responses match expectations. It allows developers to send HTTP requests without writing code, making debugging faster and more efficient. Postman supports various HTTP methods, allows easy customization of headers, parameters, and request bodies, saves requests and responses, manages authentication, handles cookies, and automates tests. Its built-in features for API documentation and sharing collections streamline development workflows.

#### Reflection Publisher-3

- This tutorial uses the push model, which sends real-time updates to subscribers without requiring them to request information. The implementation is handled in the notify function, where the publisher pushes notifications to all registered subscribers. This model is useful for instant updates in live feeds, messaging systems, or event-driven architectures, as it reduces requests and improves responsiveness.

- The pull model simplifies data retrieval by allowing subscribers to control data retrieval, but can increase CPU usage and latency, while the push model ensures immediate updates but may face delays due to processing backlogs.

- My implementation utilizes multi-threading, enabling the main thread to continue processing requests while notifications are sent in the background, enhancing system responsiveness and resilience.
