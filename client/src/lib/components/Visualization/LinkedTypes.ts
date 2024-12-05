export interface Connection {
    to: string,
    from: string,
    actor: string[]
}

export interface Path {
    path: Connection[]
}