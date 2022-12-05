program part2;
uses sysutils;
type char_array = array of char;
type stack = array[0..100] of char;
type stack_array = array[1..9] of stack;
type stacklength_array = array[1..9] of int64;
var stacks: stack_array;
var stacklengths: stacklength_array;
var n, sf, st, i, ind: int64;
var item: char;
var ip: string;

procedure AddItems(sn, sl: int64; items: char_array);
var i, amount: int64;
begin
    amount := length(items);
    for i := sl-1 downto 0 do
    begin
        stacks[sn][i+amount] := stacks[sn][i];
    end;
    for i := 0 to amount-1 do
    begin
        stacks[sn][i] := items[i]
    end;
end;

function RemoveItems(sn, sl, amount: int64): char_array;
var i: int64;
begin
    setlength(removeitems, amount);
    for i := 0 to amount-1 do
        removeitems[i] := stacks[sn][i];
    for i := amount-1 to sl-1 do
    begin
        stacks[sn][i-amount] := stacks[sn][i];
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
        additems(st, stacklengths[st], removeitems(sf, stacklengths[sf], n));
        inc(stacklengths[st], n);
        dec(stacklengths[sf], n);
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
