const assert = require('assert');

let addon = require('./dist');

assert.deepStrictEqual(addon.customJson(), {
    customFieldName: 10
}, "verify custom json");
assert.deepStrictEqual(addon.standardJson(), {
    someName: "John",
    aNumber: 1337
}, "verify standard json");
assert.deepStrictEqual(addon.multilevelJson(), {
    val: ["hello"]
}, "verify multilevel json");

assert.strictEqual(addon.unitStruct(), null);

assert.deepStrictEqual(addon.withMessage(), {
    withMessage: ["test", 321n]
}, "simple unnamed enum variant");
assert.deepStrictEqual(addon.withFields(), {
    withFields: {
        val: 123n
    }
}, "named enum variant");
assert.deepStrictEqual(addon.withUnit(),
    "UnitErrorType",
    "unit enum variant")

assert.throws(() => addon.failedResultWithFields(), {
    "withFields": {
        val: 987n
    }
}, "sync exception");

assert.rejects(() => addon.asyncResultFailedUnit(),
               (err) => {
                   assert.strictEqual(err, "UnitErrorType");
                   return true;
               },
               "async exception");

assert.deepStrictEqual(addon.withSerdeJson(), {
    val: {
        first: true,
        second: "hello"
    }
}, "serde_json serialization")