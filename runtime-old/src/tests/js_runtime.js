function sayHello(a, b) {
  console.log(a + b);
}

function main(args) {
  if (args.hasOwnProperty("a") && args.hasOwnProperty("b")) {
    // Check if arguments 'a' and 'b' exist
    sayHello(args.a, args.b);
  } else {
    // If arguments are missing, print an error message
    console.log("Error: Missing required arguments 'a' and/or 'b'.");
  }
}
