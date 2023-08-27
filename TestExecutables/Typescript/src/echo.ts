// This is a simple echo program written in Typescript
// It will echo the command line arguments back to the console

// Get the command line arguments
var args = process.argv.slice(2);

// Echo the arguments back to the console
console.log(args.join(' '));

// Exit with a success code
process.exit(0);