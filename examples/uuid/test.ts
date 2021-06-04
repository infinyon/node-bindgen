const assert = require('assert');

interface UuidExample {
    makeUuid(): string;
    takeUuid(uuid: string);
}
let addon: UuidExample = require('./dist');

const new_uuid: string = addon.makeUuid();
assert.equal(new_uuid, "f7509856-9ae5-4c07-976d-a5b3f983e4af");

addon.takeUuid(new_uuid);
