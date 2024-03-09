export type Size = {
  width: number
  height: number
}

export type GameState = "paused" | "over" | "running";

export type Game = {
  snake: {
    direction: string
    head: {
      x: number
      y: number
    }
    tail: {
      x: number
      y: number
    }[]
  }
  fruit: {
    x: number
    y: number
  }
  score: number
  state: GameState
}

export type InitOptions = {
  frameDuration: number
  size: {
    width: number
    height: number
  }
  featuresWithVersion: Record<string, string>
  metadatas: Record<string, string>
}

export type SetupFunction = (initOptions: InitOptions, gameNode: HTMLElement) => {
  context: any,
  cleanup: () => {}
}

export type RenderFrameFunction = (size: InitOptions, frameInfos: Game, context: any) => {}

export type Renderer = {
  setup: SetupFunction,
  renderFrame: RenderFrameFunction
}
