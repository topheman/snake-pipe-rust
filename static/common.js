((global) => {
  async function fetchInitOptions() {
    try {
      const res = await fetch('/init-options');
      if (res.ok) {
        return await res.json();
      }
    }
    catch (e) {
      console.error(e);
      return null;
    }
    return null;
  }

  /**
   *
   * @param {(eventName: 'connected' | 'event', payload: any) => void} cb
   */
  global.bootstrap = async function bootstrap(cb) {
    const events = new EventSource("/events");
    events.onmessage = (event) => {
      if (event.data === 'connected') {
        fetchInitOptions().then(initOptions => {
          cb('connected', JSON.parse(initOptions))
        })
      }
      else {
        cb('event', JSON.parse(event.data));
      }
    }
  }
})(window);
