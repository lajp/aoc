program part1;
uses sysutils;
type stack = array[0..100] of char;
type stack_array = array[1..9] of stack;
type stacklength_array = array[1..9] of int64;
var stacks: stack_array;
var stacklengths: stacklength_array;
var n, sf, st, i, ind: int64;
var item: char;
var ip: string;

procedure AddItem(sn: int64; item: char; sl: int64);
var i: int64;
begin
    for i := sl downto 0 do
    begin
        stacks[sn][i+1] := stacks[sn][i];
    end;
    stacks[sn][i] := item;
end;

function RemoveItem(sn, sl: int64): char;
var i: int64;
begin
    removeitem := stacks[sn][0];
    for i := 1 to sl do
    begin
        stacks[sn][i-1] := stacks[sn][i];
    end;
end;

begin
    stacks := default(stack_array);
    stacklengths := default(stacklength_array);
    readln(ip);

    while ip[2] <> '1' do
    begin
        i := 2;
        while i < length(ip) do
        begin
            if ip[i] <> ' ' then
            begin
                ind := ((i-2) div 4)+1;
                stacks[ind][stacklengths[ind]] := ip[i];
                inc(stacklengths[ind]);
            end;
            inc(i, 4);
        end;
        readln(ip);
    end;
    readln(ip);

    while not eof do
    begin
        readln(ip);
        sscanf(ip, 'move %d from %d to %d', [@n, @sf, @st]);
        for i := 1 to n do
        begin
            item := removeitem(sf, stacklengths[sf]);
            additem(st, item, stacklengths[st]);
            inc(stacklengths[st]);
            dec(stacklengths[sf]);
        end;
    end;

    for i := low(stacks) to high(stacks) do
    begin
        item := stacks[i][0];
        if item = chr(0) then
            break;
        write(item);
    end;
    writeln;
end.
