package main

import "fmt"

// #include <scs.h>
import "C"

func main() {
	fmt.Printf("%#v\n", C.GoString(C.scs_version()))
}
