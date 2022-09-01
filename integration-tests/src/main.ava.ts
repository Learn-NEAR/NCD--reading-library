import { Worker, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

const defaultParameters = {
  isbn: "123-4564-54-4",
  name: "Test",
  description: "Description test",
  numpage: 123,
  author: "juan perez",
  datepublished: "marzo 2099",
  editions: 1,
};

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount("test-account");
  // Get wasm file path from package.json test script in folder above
  await contract.deploy(process.argv[2]);
  // JavaScript contracts require calling 'init' function upon deployment
  await contract.call(contract, "init", {});

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("AddBook#shold run fine with these parameters", async (t) => {
  const { root, contract } = t.context.accounts;

  try {
    await root.call(contract, "AddBook", {
      ...defaultParameters,
    });
    t.assert(true);
  } catch {
    t.assert(false);
  }
});

test("AddBook#shold fail if we give an empty isbn", async (t) => {
  const { root, contract } = t.context.accounts;

  try {
    await root.call(contract, "AddBook", {
      ...defaultParameters,
      isbn: "",
    });
    t.assert(false);
  } catch {
    t.assert(true);
  }
});

test("AddBook#shold fail if we give a empty name", async (t) => {
  const { root, contract } = t.context.accounts;

  try {
    await root.call(contract, "AddBook", {
      ...defaultParameters,
      name: "",
    });
    t.assert(false);
  } catch {
    t.assert(true);
  }
});

test("AddBook#shold fail if we give a empty description", async (t) => {
  const { root, contract } = t.context.accounts;

  try {
    await root.call(contract, "AddBook", {
      ...defaultParameters,
      description: "",
    });
    t.assert(false);
  } catch {
    t.assert(true);
  }
});

test("AddBook#shold fail if we give 0 numpage", async (t) => {
  const { root, contract } = t.context.accounts;

  try {
    await root.call(contract, "AddBook", {
      ...defaultParameters,
      numpage: 0,
    });
    t.assert(false);
  } catch {
    t.assert(true);
  }
});

test("getBook#should return insterted book", async (t) => {
  const { root, contract } = t.context.accounts;

  await root.call(contract, "AddBook", defaultParameters);

  const book = await contract.view("getBook", { id: 0 });

  t.truthy(book);
});

test("getBook#should fail when calling with negative id", async (t) => {
  const { root, contract } = t.context.accounts;

  await root.call(contract, "AddBook", defaultParameters);

  try {
    await contract.view("getBook", { id: -1 });
    t.assert(false);
  } catch {
    t.assert(true);
  }
});

test("getBook#should fail when calling with out of bounds id", async (t) => {
  const { root, contract } = t.context.accounts;

  await root.call(contract, "AddBook", defaultParameters);

  try {
    await contract.view("getBook", { id: 1600 });
    t.assert(false);
  } catch {
    t.assert(true);
  }
});
