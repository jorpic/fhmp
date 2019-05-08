
const config = {
  // Draft note will be saved to the storage every now and then
  DRAFT_SAVE_TIMEOUT: 4000, // µseconds
  SYNC_SERVER_URL: "",
  CLIENT_KEY: "",
};


// Loads config into global variable
export function loadConfig(db) {
  return db.loadConfig().then(cfg => {
    if (cfg) {
      for (let k in config)
        if (k in cfg) config[k] = cfg[k];
    }
  });
}


// Saves global config into IDB
export function saveConfig(db, cfg) {
  return db.saveConfig(cfg).then(() => {
    for (let k in config)
      if (k in cfg) config[k] = cfg[k];
  });
}


export default config;
