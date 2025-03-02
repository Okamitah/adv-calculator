def separate_expression(expression: str):

    expression = expression.replace(" ","")

    ops = ['+', '-','*', '/']

    ls = split_ops(expression, ops)

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

    addsub = ('+','-') # 2*2*2 + 100
    muldiv = ('*','/')

    for i in range(1,len(expression),2):

        if expression[i] in muldiv:
            expression[i+1] = operations_map[expression[i]](expression[i+1],expression[i-1])
            expression.pop(i-1)
            expression.pop(i)
        elif expression[i] in addsub:
            expression.append(expression[i-1])
            expression.append(expression[i])
    
    for i in range(1,len(expression),2):
        expression[i+1] = operations_map[expression[i]](expression[i-1],expression[i+1])
    return expression[-1]


def calculate(expression: str):
    separated_expression = separate_expression(expression)

    for index,expression in enumerate(separated_expression):
        if type(expression) == list:
            separated_expression[index] = calculate_list(expression)
    return calculate_list(separated_expression)



print("\nWelcome to my basic calculator!\n")
print("Allowed operators: +, -, * and /\n")
exp = ""
while exp != "exit":
    exp = input("> ")
    print(calculate(exp))
