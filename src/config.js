
const config = {
  // Draft note will be saved to the storage every now and then
  DRAFT_SAVE_TIMEOUT: 4000, // µseconds
  SERVER_URL: null,
  CLIENT_KEY: null,
};


// Loads config into global variable
export function loadConfig(db) {
  return db.loadConfig().then(cfg => {
    if (cfg) {
      for (let k in config) config[k] = cfg[k];
    }
  });
}


// Saves global config into IDB
export function saveConfig(db) {
  return db.saveConfig(config);
}


export default config;
