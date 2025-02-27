def separate_expression(expression: str):

    expression = expression.replace(" ","")

    addsub = ['+', '-']
    muldiv = ['*', '/']

    ls = split_ops(expression, addsub)

    for el in range(len(ls)):
        if any(element in ls[el] for element in muldiv):
            ls[el] = split_ops(ls[el], muldiv)

    return ls


def split_ops(expression, operations):

    expression_list = []
    sep = 0

    for index, character in enumerate(expression):
        if character in operations:
            expression_list.append(expression[sep:index])
            expression_list.append(character)
            sep = index+1 

    expression_list.append(expression[sep:])

    return expression_list


operations_map = {
    '+': lambda x,y : float(x) + float(y),
    '-': lambda x,y : float(x) - float(y),
    '*': lambda x,y : float(x) * float(y),
    '/': lambda x,y : float(y) / float(x),
}


def calculate_list(expression: list):

    for i in range(1,len(expression),2):
        expression[i+1] = operations_map[expression[i]](expression[i+1],expression[i-1])

    return expression[-1]


def calculate(separated_expression: list):

    for index,expression in enumerate(separated_expression):
        if type(expression) == list:
            separated_expression[index] = calculate_list(expression)
    return calculate_list(separated_expression)


operation = "  2+3-  4*2 +2+2-3/2*4*  5+3 -2-9/     7+4-2"


print("\n",operation)
print("\nSeparating:\n")
print(separate_expression(operation))
print("\nCalculating:\n")
print(calculate(separate_expression(operation)))
exp = ""
while exp != "exit":
    exp = input("> ")
    print(calculate(separate_expression(exp)))
