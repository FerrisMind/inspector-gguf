import json
import subprocess
import sys

def run_cmd(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.stdout, result.stderr, result.returncode

# Распаковываем профиль
print("Распаковываем профиль...")
stdout, stderr, code = run_cmd('powershell -c "Add-Type -AssemblyName System.IO.Compression.FileSystem; [System.IO.Compression.GZipStream]::new([System.IO.File]::OpenRead(\'optimized-profile.json.gz\'), [System.IO.Compression.CompressionMode]::Decompress) | Out-File -FilePath \'optimized-profile.json\' -Encoding UTF8"')

if code != 0:
    print("Ошибка распаковки:", stderr)
    sys.exit(1)

# Анализируем профиль
print("Анализируем профиль...")
with open('optimized-profile.json', 'r', encoding='utf-8-sig') as f:
    data = json.load(f)

print('=== АНАЛИЗ ОПТИМИЗИРОВАННОГО ПРОФИЛЯ ===')
print()

threads = data['threads']
print(f'Всего потоков: {len(threads)}')
print()

total_samples = 0
total_cpu_time = 0

for i, thread in enumerate(threads):
    name = thread.get('name', f'Thread {i}')
    is_main = thread.get('isMainThread', False)
    samples = thread.get('samples', {})
    sample_count = samples.get('length', 0)

    print(f'Поток {i+1}: {name}')
    print(f'  Основной поток: {is_main}')
    print(f'  Количество семплов: {sample_count}')

    if sample_count > 0:
        time_deltas = samples.get('timeDeltas', [])
        cpu_deltas = samples.get('threadCPUDelta', [])

        total_time = sum(time_deltas)
        total_cpu = sum(cpu_deltas)

        print(f'  Общее время: {total_time:.2f} мс')
        print(f'  CPU время: {total_cpu} мкс')

        total_samples += sample_count
        total_cpu_time += total_cpu

        # Анализ функций
        stack_table = thread.get('stackTable', {})
        frame_table = thread.get('frameTable', {})

        if stack_table and 'stack' in samples:
            stacks = samples['stack']
            func_table = thread.get('funcTable', {})
            func_names = func_table.get('name', [])

            if stacks and func_names:
                func_calls = []
                for stack_idx in stacks:
                    if stack_idx is not None and stack_idx < len(stack_table.get('frame', [])):
                        frame_idx = stack_table['frame'][stack_idx]
                        if frame_idx < len(frame_table.get('func', [])):
                            func_idx = frame_table['func'][frame_idx]
                            if func_idx < len(func_names):
                                func_calls.append(func_names[func_idx])

                if func_calls:
                    from collections import Counter
                    func_counter = Counter(func_calls)
                    most_common = func_counter.most_common(5)
                    print('  Топ-5 функций по семплам:')
                    for func_name, count in most_common:
                        print(f'    {func_name}: {count}')

    print()

print(f'=== СВОДКА ===')
print(f'Общее количество семплов: {total_samples}')
print(f'Общее CPU время: {total_cpu_time} мкс ({total_cpu_time/1000:.2f} мс)')
print()

# Сравнение с предыдущим профилем
print('=== СРАВНЕНИЕ С ПРЕДЫДУЩИМ ПРОФИЛЕМ ===')
print('Предыдущий профиль (с 60-секундной задержкой):')
print('  CPU время: 9087 мкс (9.09 мс)')
print('  Основное узкое место: NtDelayExecution (ожидание)')
print()
print('Оптимизированный профиль (без блокировки):')
print(f'  CPU время: {total_cpu_time} мкс ({total_cpu_time/1000:.2f} мс)')
print('  Улучшение: ~' + str(int((9087 - total_cpu_time) / 9087 * 100)) + '% снижение CPU времени')
print()

print('=== ОПТИМИЗАЦИЯ ЗАВЕРШЕНА ===')
