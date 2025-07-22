export type Result = {
    name: string;
    value: number;
};


export type ElectionResult = {
    name: string;
    CDU: number;
    SPD: number;
    GRÜNE: number;
    LINKE: number;
    FDP: number;
    AfD: number;
    Sonstige: number;
};

export interface Election {
    id: number;
    name: string;
    date: number;
    region: Region[];
}

interface Region {
    id: number;
    name: string;
    turnout: Turnout[];
    votes: Vote[];
}

interface Turnout {
    eligible: number,
    primary_vote: boolean,
    voted: number,
    turnout: number
}
interface Vote {
    id: string,
    name: string,
    votes: number,
    primary_vote: boolean,
    percentage: number
}