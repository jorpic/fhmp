import Dexie from "dexie";
import config from "./config";


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

  // There is only one config and it has id = 0.
  loadConfig = () => this.idb.Config.get(0)
  saveConfig = cfg => this.idb.Config.put({id: 0, ...cfg})


  saveDraft = text => {
    const Drafts = this.idb.Drafts;
    // Delete all previous drafts after successfully adding new one.
    return Drafts.toCollection().primaryKeys()
      .then(keys =>
        Drafts.add({text})
          .then(() => Drafts.bulkDelete(keys)));
  }

  getDraft = () => this.idb.Drafts.toCollection().last()


  createNote = text => {
    // We use timestamp with random suffix as unique note id.
    // Couple of base-36 digits should be enough to prevent collisions
    // with a dosen of devices.
    const id = Date.now().toString(36)
      + Math.random().toString(36).substring(1,4);
    const created_at = new Date().toISOString();
    // Review this note as soon as possible.
    const review_at = created_at;
    return this.idb.Notes.add({id, review_at, created_at, text})
      .then(() => this.idb.Drafts.clear());
  }

  updateNode = () => Promise.reject("not implemented")


  // Fetch notes to review.
  getNotesToReview = () => {
    const now = new Date().toISOString();
    return this.idb.Notes.where("review_at")
      .below(now)
      .limit(config.QUEUE_LIMIT)
      .toArray()
      .then(shuffle);
  }
}


// Copy-pasted from https://stackoverflow.com/questions/2450954.
function shuffle(array) {
  for (let i = array.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [array[i], array[j]] = [array[j], array[i]];
  }
  return array;
}
