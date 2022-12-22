# rot64
This little terminal program implements an elementary cellular automata using bitwise operations (especially cyclic rotation) on u64 integers. 

Each cell is updated according to the states of its 3 left neighbors and 3 right neighbors, so that each rule requires 2^6 = 64 bits. 

This makes it easy to use the rule to seed the first row, so that we watch the rule transform itself as an example of its style. 

I also did a version that uses Raylib for output.
![rot64_A](https://user-images.githubusercontent.com/90075803/209187769-3a574870-2a15-4dcc-8bdd-77080227474d.png)

![rot64_B](https://user-images.githubusercontent.com/90075803/209187992-529d44b2-5ef1-484a-93d3-2f8ef8fe76aa.png)
![rot64_C](https://user-images.githubusercontent.com/90075803/209188032-e3f0d553-0ebb-476b-a0c9-3876d94fa9f8.png)



