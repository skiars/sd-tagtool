export interface TagData {
  key: number,
  name: string,
  url: string,
  tags: string[]
}

export interface EditAction {
  index: number,
  tags: string[]
}

export enum FilterMode {
  IncludeAny,
  IncludeAll,
  Exclude
}

export class EditorHistory {
  constructor(dataset: TagData[] = []) {
    this.dataset = dataset
  }

  public edit(actions: EditAction[]): TagData[] | undefined {
    if (actions.length) {
      let undo: EditAction[] = []
      actions.forEach(x => {
        undo.push({index: x.index, tags: this.dataset[x.index].tags})
        this.dataset[x.index].tags = x.tags
      })
      this.undoStack.push(undo)
      this.redoStack = []
      return this.dataset
    }
    return undefined
  }

  public undo(): TagData[] | undefined {
    if (this.undoStack.length) {
      let e: EditAction[] = []
      this.undoStack.pop()?.forEach(x => {
        e.push({index: x.index, tags: [...this.dataset[x.index].tags]})
        this.dataset[x.index].tags = x.tags
      })
      this.redoStack.push(e)
      return this.dataset
    }
    return undefined
  }

  public redo(): TagData[] | undefined {
    if (this.redoStack.length) {
      let e: EditAction[] = []
      this.redoStack.pop()?.forEach(x => {
        e.push({index: x.index, tags: [...this.dataset[x.index].tags]})
        this.dataset[x.index].tags = x.tags
      })
      this.undoStack.push(e)
      return this.dataset
    }
    return undefined
  }

  public state(): EditAction[] | undefined {
    if (this.undoStack.length)
      return this.undoStack[this.undoStack.length - 1]
    return undefined
  }

  public dataset: TagData[]
  private undoStack: EditAction[][] = []
  private redoStack: EditAction[][] = []
}

function removeDuplicates<T>(x: T[]): T[] {
  let ts: Set<T> = new Set
  return x.filter(x => x && !ts.has(x) && ts.add(x))
}

export function collectTags(dataset: TagData[]): string[] {
  return removeDuplicates(dataset.flatMap(x => x.tags))
}

export function deleteTags(dataset: TagData[], tags: string[]): EditAction[] {
  let del: Set<string> = new Set(tags)
  let edited: EditAction[] = []
  dataset.forEach(x => {
    const deleted = x.tags.filter(x => !del.has(x))
    if (deleted.length < x.tags.length) {
      edited.push({
        index: x.key,
        tags: deleted
      })
    }
  })
  return edited
}

export function insertTags(dataset: TagData[],
                           tags: string[],
                           position: number | undefined): EditAction[] {
  tags = removeDuplicates(tags)
  if (!tags.length)
    return []
  if (typeof (position) != 'number') { // auto mode
    return dataset.map(x => {
      const ts: Set<string> = new Set(x.tags)
      return {index: x.key, tags: x.tags.concat(tags.filter(a => !ts.has(a)))}
    })
  }
  const ts: Set<string> = new Set(tags)
  return dataset.map(x => {
    let s1: string[] = x.tags.filter(a => !ts.has(a))
    let s2: string[] = []
    if (position >= 0)
      s2 = s1.splice(position, s1.length)
    else
      s2 = s1.splice(s1.length + position, s1.length)
    return {index: x.key, tags: s1.concat(tags).concat(s2)}
  })
}

export function replaceTags(dataset: TagData[], from: string[], to: string[]): EditAction[] {
  from = removeDuplicates(from)
  to = removeDuplicates(to)
  if (!from.length)
    return [] //  do nothing
  if (!to.length)
    return deleteTags(dataset, from)
  let repl: Map<string, string[]> = new Map
  let n = Math.min(from.length, to.length)
  for (let i = 0; i < n; i++)
    repl.set(from[i], [to[i]])
  for (; n < from.length; n++)
    repl.set(from[n], [])
  if (to.length > from.length)
    repl.set(from[from.length - 1], to.slice(from.length - 1))
  let edited: EditAction[] = []
  dataset.forEach(x => {
    let replaced: string[] = []
    let edit = false
    x.tags.forEach(x => {
      const r = repl.get(x)
      if (r != undefined) {
        edit = true
        replaced = replaced.concat(r)
      } else
        replaced.push(x)
    })
    if (edit) {
      replaced = removeDuplicates(replaced)
      edited.push({
        index: x.key,
        tags: replaced
      })
    }
  })
  return edited
}
