import Dexie from "dexie";

const DB_VERSION = 4;

export default class Db {
  constructor() {
    let ver = 0;
    this.idb = new Dexie("fhmp");
    this.idb.version(++ver).stores({
      Notes: "++id,createTime",
    });
    this.idb.version(++ver).stores({
      Notes: "++id,createTime",
      Drafts: "++id",
    });
    this.idb.version(++ver).stores({
      Notes: "++id,createTime",
      Drafts: "++id",
      Config: "id",
    });
    this.idb.version(++ver).stores({
      Config: "id",
      Drafts: "++id",
      Notes: "id,review_at",
      Reviews: "",
    });
    console.assert(ver === DB_VERSION, "DB version mismatch");
  }


  open = () => this.idb.open()


  createNote = text => {
    // We use timestamp with random suffix as unique note id.
    // Couple of base-36 digits should be enough to prevent collisions
    // with a dosen of devices.
    const id = Date.now().toString(36)
      + Math.random().toString(36).substring(1,4);
    const created_at = new Date().toISOString();
    // review this note as soon as possible
    const review_at = created_at;
    return this.idb.Notes.add({id, review_at, created_at, text})
      .then(() => this.idb.Drafts.clear());
  }


  // There is only one config and it has id = 0.
  loadConfig = () => this.idb.Config.get(0)
  saveConfig = cfg => this.idb.Config.put({id: 0, ...cfg})


  // FIXME: take oldest 100 by last acess time, select random among them
  getRandomNote = () =>
    this.idb.Notes.toArray()
      .then(ns => {
        const i = Math.floor(ns.length * Math.random());
        return Promise.resolve(ns[i]);
      })


  // TODO: filter by tags
  getNotes = () => this.idb.Notes.toArray()

  updateNode = () => Promise.reject("not implemented")


  saveDraft = text => {
    const Drafts = this.idb.Drafts;
    return Drafts.toCollection().primaryKeys()
      .then(keys =>
        Drafts.add({text})
          .then(() => Drafts.bulkDelete(keys)));
  }

  getDraft = () => this.idb.Drafts.toCollection().last()

  // - addReview(db, id, review) -> result
  // - editNote(db, id, note) -> result
}
